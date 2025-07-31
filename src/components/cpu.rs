use self::csr::{CsrAddr, CsrFile};
pub use self::exec_engine::exec_unit::alu::AluControl;
pub use self::exec_engine::exec_unit::mul::MulControl;
use self::exec_engine::{
    CommonDataBus, ExecUnit, OperationType, ReorderBuffer, RobIndex, Scheduler, Schedulers,
};
use self::flip_flop::{Clearable, FlipFlop, Sequential, StallingFlipFlop};
use self::front_end::{
    BranchPredictor, DecodeQueue, Decoder, JumpAgu, PcAdder, ZeroBubblePredictor,
};
use self::mem_subsystem::LoadQueue;
use self::reg::pipeline::{BpRegs, IfRegs, ZbpRegs};
use self::reg::{RegData, RegFile, RegName};
use super::memory::{Address, MemHierarchy, MemSize, Memory};
use crate::config::Immutable;
use crate::instr::{EnvTrap, SyscallCode};
use crate::os::Os;

use std::error::Error;
use std::io::{Read, Write};
use std::mem;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

pub(crate) mod csr;
pub(super) mod diagnostics;
pub mod error;
mod exec_engine;
mod flip_flop;
mod front_end;
mod mem_subsystem;
pub(crate) mod reg;

//TODO Make configurable
const FETCH_WIDTH: NonZeroUsize = NonZeroUsize::new(4).unwrap();
const ZBP_CAPACITY: NonZeroUsize = NonZeroUsize::new(1024).unwrap();
const BP_CAPACITY: NonZeroUsize = NonZeroUsize::new(MemSize(16).KiB()).unwrap();
const DECODE_QUEUE_CAPACITY: NonZeroUsize = NonZeroUsize::new(16).unwrap();
const ISSUE_WIDTH: Immutable<NonZeroUsize> = Immutable::new(NonZeroUsize::new(4).unwrap());
const COMMIT_WIDTH: Immutable<NonZeroUsize> = Immutable::new(NonZeroUsize::new(4).unwrap());
const ROB_CAPACITY: NonZeroUsize = NonZeroUsize::new(96).unwrap();
const SCHEDULER_CAPACITY: NonZeroUsize = NonZeroUsize::new(16).unwrap();
const LOAD_QUEUE_CAPACITY: NonZeroUsize = NonZeroUsize::new(20).unwrap();
const EXEC_UNITS: [OperationType; 5] = [
    OperationType::ALU,
    OperationType::ALU.union(OperationType::MUL),
    OperationType::BRANCH,
    OperationType::LOAD,
    OperationType::STORE,
];

type Stall = bool;
type Pc = Address;
pub type ExitCode = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuRun {
    Exit(ExitCode),
    Break,
}

#[derive(Debug, Default)]
struct CycleResult {
    traps: Vec<EnvTrap>,
    errors: Vec<Box<dyn Error>>,
    jump_to_self: bool,
}

pub struct Cpu<I, O, E> {
    // Front End
    pc: StallingFlipFlop<Pc>,
    pc_adders: Box<[PcAdder]>,
    zb_predictor: ZeroBubblePredictor,
    zbp_regs: StallingFlipFlop<Box<[ZbpRegs]>>,
    if_regs: StallingFlipFlop<Box<[IfRegs]>>,
    branch_predictor: BranchPredictor,
    bp_regs: StallingFlipFlop<Box<[BpRegs]>>,
    decoders: Box<[Decoder]>,
    jump_agus: Box<[JumpAgu]>,
    decode_queue: DecodeQueue,

    // Execution Engine
    issue_width: Immutable<NonZeroUsize>,
    commit_width: Immutable<NonZeroUsize>,
    serializing: FlipFlop<bool>,
    rob: ReorderBuffer,
    schedulers: Schedulers,
    regs: RegFile,
    csr_file: CsrFile,
    exec_units: Box<[ExecUnit]>,
    cdb: CommonDataBus,

    // Memory Subsystem
    load_queue: LoadQueue,
    mem_hierarchy: MemHierarchy,

    // Misc
    cycle_result: CycleResult,
    os: Os<I, O, E>,
    exit: bool,
}

impl<I: Read, O: Write, E: Write> Cpu<I, O, E> {
    pub fn new(mem: Arc<Mutex<Memory>>, os: Os<I, O, E>) -> Self {
        let pc_adders = vec![PcAdder::new(); FETCH_WIDTH.get()].into_boxed_slice();
        //TODO Get schedulers & exec units from config
        let schedulers: Schedulers = EXEC_UNITS
            .into_iter()
            .map(|op| Scheduler::new(op, SCHEDULER_CAPACITY))
            .collect();
        let exec_units = Vec::from(&schedulers).into_boxed_slice();
        let mem_hierarchy = MemHierarchy::new(mem);

        Self {
            pc: StallingFlipFlop::new(Pc::default()),
            pc_adders,
            zb_predictor: ZeroBubblePredictor::with_capacity(ZBP_CAPACITY),
            zbp_regs: StallingFlipFlop::new(vec![].into_boxed_slice()),
            branch_predictor: BranchPredictor::with_capacity(BP_CAPACITY),
            bp_regs: StallingFlipFlop::new(vec![].into_boxed_slice()),
            if_regs: StallingFlipFlop::new(vec![].into_boxed_slice()),
            decoders: vec![Decoder::new(); FETCH_WIDTH.get()].into_boxed_slice(),
            jump_agus: vec![JumpAgu::new(); FETCH_WIDTH.get()].into_boxed_slice(),
            decode_queue: DecodeQueue::with_capacity(DECODE_QUEUE_CAPACITY),

            issue_width: ISSUE_WIDTH,
            commit_width: COMMIT_WIDTH,
            serializing: FlipFlop::new(Default::default()),
            rob: ReorderBuffer::with_capacity(ROB_CAPACITY),
            schedulers,
            regs: Default::default(),
            csr_file: Default::default(),
            exec_units,
            cdb: CommonDataBus::new(),

            load_queue: LoadQueue::with_capacity(LOAD_QUEUE_CAPACITY),
            mem_hierarchy,

            cycle_result: Default::default(),
            os,
            exit: false,
        }
    }

    pub fn run(&mut self) -> Result<CpuRun, Vec<Box<dyn Error>>> {
        loop {
            let CycleResult {
                traps,
                mut errors,
                jump_to_self,
            } = self.clock_cycle();
            let mut brk = false;
            for trap in traps {
                match trap {
                    EnvTrap::Ecall => self.handle_syscall(),
                    EnvTrap::Ebreak => brk = true,
                    EnvTrap::Exception(ex) => {
                        //TODO Exception handling
                        errors.push(ex.into())
                    }
                }
            }

            return if !errors.is_empty() {
                Err(errors)
            } else if jump_to_self && self.exit {
                Ok(CpuRun::Exit(self.get_exit_code()))
            } else if brk {
                Ok(CpuRun::Break)
            } else {
                continue;
            };
        }
    }

    pub fn step(&mut self) -> Result<CpuRun, Vec<Box<dyn Error>>> {
        let CycleResult {
            traps,
            mut errors,
            jump_to_self,
        } = self.clock_cycle();
        for trap in traps {
            match trap {
                EnvTrap::Ecall => self.handle_syscall(),
                EnvTrap::Ebreak => {}
                EnvTrap::Exception(ex) => {
                    //TODO Exception handling
                    errors.push(ex.into())
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else if jump_to_self && self.exit {
            Ok(CpuRun::Exit(self.get_exit_code()))
        } else {
            Ok(CpuRun::Break)
        }
    }

    fn handle_syscall(&mut self) {
        macro_rules! os_call {
            ($os:expr, $mem:expr, |os| os.$( $call:tt )+) => {
                crate::components::cpu::reg::RegData::signed(match $os.$( $call )+ {
                    Ok(r) => r,
                    Err((r, errno)) => {
                        $os.set_errno(&mut $mem, errno);
                        r
                    }
                } as _)
            };
        }

        let os = &mut self.os;
        let reg_file = &mut self.regs;
        let mut mem = self.mem_hierarchy.l1d();
        // TODO Log unknown syscalls instead of panicking
        let syscall = SyscallCode::try_from(reg_file.get(RegName::A7)).unwrap();
        match syscall {
            SyscallCode::Exit => self.exit = true,
            SyscallCode::Close => {
                let fd = reg_file.get(RegName::A0).i() as _;
                let result = os_call!(os, mem, |os| os.close(fd));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Fstat => {
                let fd = reg_file.get(RegName::A0).i() as _;
                let statbuf = reg_file.get(RegName::A1).addr();
                let result = os_call!(os, mem, |os| os.fstat(&mut mem, fd, statbuf));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Lseek => {
                let fd = reg_file.get(RegName::A0).i() as _;
                let offset = reg_file.get(RegName::A1).i() as _;
                let whence = reg_file.get(RegName::A2).i() as _;
                let result = os_call!(os, mem, |os| os.lseek(fd, offset, whence));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Read => {
                let fd = reg_file.get(RegName::A0).i() as _;
                let buf = reg_file.get(RegName::A1).addr();
                let count = reg_file.get(RegName::A2).u() as _;
                let result = os_call!(os, mem, |os| os.read(&mut mem, fd, buf, count));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Sbrk => {
                let incr = reg_file.get(RegName::A0).i() as _;
                let sp = reg_file.get(RegName::SP).addr();
                let result = os_call!(os, mem, |os| os.sbrk(sp, incr));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Write => {
                let fd = reg_file.get(RegName::A0).i() as _;
                let buf = reg_file.get(RegName::A1).addr();
                let count = reg_file.get(RegName::A2).u() as _;
                let result = os_call!(os, mem, |os| os.write(&mut mem, fd, buf, count));
                reg_file.set(RegName::A0, result);
            }
        }
    }
}

impl<I, O, E> Cpu<I, O, E> {
    pub fn scheduler_count(&self) -> usize {
        self.schedulers.count()
    }

    pub fn os(&mut self) -> &mut Os<I, O, E> {
        &mut self.os
    }

    pub(crate) unsafe fn init(&mut self, entrypoint: Address, sp: Address) {
        self.pc.write(entrypoint);
        self.regs.set(RegName::SP, RegData::address(sp));
        self.csr_file.implicit_write(CsrAddr::MCYCLE, 0);
        self.csr_file.implicit_write(CsrAddr::MINSTRET, 0);

        self.pc.finish_cycle();
        self.regs.future_file_mut().clear();
        self.regs.finish_cycle();
        self.csr_file.finish_cycle();
    }

    pub fn reset(&mut self) {
        self.pc.clear();
        self.flush_pipeline();
    }

    fn clock_cycle(&mut self) -> CycleResult {
        let cycle = self.csr_file.implicit_read(CsrAddr::MCYCLE).wrapping_add(1);
        self.csr_file.implicit_write(CsrAddr::MCYCLE, cycle);

        let new_pc = self.zb_predict();
        let fetch_stall = self.fetch();
        let bp_jump = self.branch_prediction();
        let decode_result = self.decode();
        self.issue();
        self.execute();
        self.write_result();
        let mispredicted = self.commit();

        // PC multiplexer & pipeline register control signals
        if let Some(jump) = mispredicted {
            self.flush_pipeline();
            self.pc.write(jump);
        } else if let Err(jump) = decode_result {
            self.zbp_regs.clear();
            self.if_regs.clear();
            self.bp_regs.clear();
            self.pc.write(jump)
        } else if let Ok(true) = decode_result {
            self.pc.stall();
            self.zbp_regs.stall();
            self.if_regs.stall();
            self.bp_regs.stall();
        } else if let Some(jump) = bp_jump {
            self.zbp_regs.clear();
            self.if_regs.clear();
            self.pc.write(jump);
        } else if fetch_stall {
            self.pc.stall();
            self.zbp_regs.stall();
            self.if_regs.clear();
        } else {
            self.pc.write(new_pc);
        };
        self.finish_cycle();

        mem::take(&mut self.cycle_result)
    }

    fn flush_pipeline(&mut self) {
        self.zbp_regs.clear();
        self.if_regs.clear();
        self.bp_regs.clear();
        self.decode_queue.clear();
        self.serializing.clear();
        self.rob.clear();
        self.schedulers.clear();
        for exec_unit in &mut self.exec_units {
            exec_unit.clear();
        }
        self.regs.future_file_mut().clear();
        self.cdb.clear();
        self.load_queue.clear();
    }

    fn get_exit_code(&self) -> ExitCode {
        self.regs.get(RegName::A0).i() as _
    }
}

impl<I, O, E> Sequential for Cpu<I, O, E> {
    fn finish_cycle(&mut self) {
        self.pc.finish_cycle();
        self.zb_predictor.finish_cycle();
        self.zbp_regs.finish_cycle();
        self.if_regs.finish_cycle();
        self.branch_predictor.finish_cycle();
        self.bp_regs.finish_cycle();
        self.decode_queue.finish_cycle();
        self.serializing.finish_cycle();
        self.rob.finish_cycle();
        self.schedulers.finish_cycle();
        for exec_unit in &mut self.exec_units {
            exec_unit.finish_cycle();
        }
        self.regs.finish_cycle();
        self.csr_file.finish_cycle();
        self.cdb.finish_cycle();
        self.load_queue.finish_cycle();
    }
}
