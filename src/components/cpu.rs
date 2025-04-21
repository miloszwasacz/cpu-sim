pub use self::exec_engine::exec_unit::alu::AluControl;
use self::exec_engine::{
    CommonDataBus, ExecUnit, OperationType, ReorderBuffer, RobIndex, Scheduler, Schedulers,
};
use self::flip_flop::{Clearable, Sequential, StallingFlipFlop};
use self::front_end::{Decoder, JumpAgu, PcAdder};
use self::mem_subsystem::LoadQueue;
use self::reg::pipeline::{IdIsRegs, IfIdRegs};
use self::reg::{RegFile, RegName};
use super::memory::{Address, Memory};
use super::Bus;
use crate::instr::{EnvTrap, SyscallCode};
use crate::os::Os;

use std::error::Error;
use std::mem;
use std::num::NonZeroUsize;

pub(super) mod diagnostics;
pub mod error;
mod exec_engine;
mod flip_flop;
mod front_end;
mod mem_subsystem;
pub(crate) mod reg;

//TODO Make configurable
const ROB_CAPACITY: NonZeroUsize = NonZeroUsize::new(96).unwrap();
const SCHEDULER_CAPACITY: NonZeroUsize = NonZeroUsize::new(16).unwrap();
const LOAD_QUEUE_CAPACITY: NonZeroUsize = NonZeroUsize::new(20).unwrap();

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

pub struct Cpu<'m> {
    // Front End
    pc: StallingFlipFlop<Pc>,
    pc_adder: PcAdder,
    instr_mem: Bus<'m, Memory>,
    if_id_regs: StallingFlipFlop<Option<IfIdRegs>>,
    decoder: Decoder,
    jump_agu: JumpAgu,
    id_is_regs: StallingFlipFlop<Option<IdIsRegs>>,

    // Execution Engine
    rob: ReorderBuffer,
    schedulers: Schedulers,
    regs: RegFile,
    exec_units: Vec<ExecUnit>,
    cdb: CommonDataBus,

    // Memory Subsystem
    load_queue: LoadQueue,
    data_mem: Bus<'m, Memory>,

    // Misc
    cycle_result: CycleResult,
    os: Os,
    exit: bool,
}

impl<'m> Cpu<'m> {
    pub fn new(mem_bus: Bus<'m, Memory>) -> Self {
        //TODO Get schedulers & exec units from config
        let schedulers: Schedulers = [
            OperationType::ALU,
            OperationType::BRANCH,
            OperationType::LOAD,
            OperationType::STORE,
        ]
        .into_iter()
        .map(|op| Scheduler::new(op, SCHEDULER_CAPACITY))
        .collect();
        let exec_units = (&schedulers).into();

        Self {
            pc: StallingFlipFlop::new(Pc::default()),
            pc_adder: PcAdder::new(),
            instr_mem: mem_bus,
            if_id_regs: StallingFlipFlop::new(None),
            decoder: Decoder::new(),
            jump_agu: JumpAgu::new(),
            id_is_regs: StallingFlipFlop::new(None),
            rob: ReorderBuffer::with_capacity(ROB_CAPACITY),
            schedulers,
            regs: Default::default(),
            exec_units,
            cdb: CommonDataBus::new(),
            load_queue: LoadQueue::with_capacity(LOAD_QUEUE_CAPACITY),
            data_mem: mem_bus,
            cycle_result: Default::default(),
            os: Os::new(),
            exit: false,
        }
    }

    pub fn scheduler_count(&self) -> usize {
        self.schedulers.count()
    }

    pub(crate) fn os(&mut self) -> &mut Os {
        &mut self.os
    }

    pub(crate) unsafe fn set_pc(&mut self, entrypoint: Address) {
        self.pc = StallingFlipFlop::new(entrypoint);
    }

    pub fn reset(&mut self) {
        self.pc.clear();
        self.flush_pipeline();
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
                    EnvTrap::Syscall => self.handle_syscall(),
                    EnvTrap::Break => brk = true,
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
                EnvTrap::Syscall => self.handle_syscall(),
                EnvTrap::Break => {}
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

    fn clock_cycle(&mut self) -> CycleResult {
        let pc_plus_4 = self.fetch();
        let jump = self.decode();
        let stall = self.issue();
        self.execute();
        self.write_result();
        let mispredicted = self.commit();

        // PC MUX
        match mispredicted {
            Some(jump) => {
                self.pc.write(jump);
                self.flush_pipeline();
            }
            None if stall => {
                self.pc.stall();
                self.if_id_regs.stall();
                self.id_is_regs.stall();
            }
            None => self.pc.write(match jump {
                Some(jump) => {
                    self.if_id_regs.clear();
                    jump
                }
                None => pc_plus_4,
            }),
        }
        self.finish_cycle();

        mem::take(&mut self.cycle_result)
    }

    fn flush_pipeline(&mut self) {
        //TODO Flush as early as possible (i.e. before commit)
        //     and clear only entries after the mispredicted one
        self.if_id_regs.clear();
        self.id_is_regs.clear();
        self.rob.clear();
        self.schedulers.clear();
        self.regs.future_file_mut().clear();
        self.cdb.clear();
        self.load_queue.clear();
    }

    fn handle_syscall(&mut self) {
        macro_rules! os_call {
            ($os:expr, $mem:expr, |os| os.$( $call:tt )+) => {
                crate::components::cpu::reg::RegData::signed(match $os.$( $call )+ {
                    Ok(r) => r,
                    Err((r, errno)) => {
                        $os.set_errno($mem, errno);
                        r
                    }
                })
            };
        }

        let os = &mut self.os;
        let reg_file = &mut self.regs;
        let mem = &mut self.data_mem.borrow_mut();
        // TODO Log unknown syscalls instead of panicking
        let syscall = SyscallCode::try_from(reg_file.get(RegName::A7)).unwrap();
        match syscall {
            SyscallCode::Exit => self.exit = true,
            SyscallCode::Close => {
                let fd = reg_file.get(RegName::A0).i();
                let result = os_call!(os, mem, |os| os.close(fd));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Fstat => {
                let fd = reg_file.get(RegName::A0).i();
                let statbuf = reg_file.get(RegName::A1).addr();
                let result = os_call!(os, mem, |os| os.fstat(mem, fd, statbuf));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Lseek => {
                let fd = reg_file.get(RegName::A0).i();
                let offset = reg_file.get(RegName::A1).i();
                let whence = reg_file.get(RegName::A2).i();
                let result = os_call!(os, mem, |os| os.lseek(fd, offset, whence));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Read => {
                let fd = reg_file.get(RegName::A0).i();
                let buf = reg_file.get(RegName::A1).addr();
                let count = reg_file.get(RegName::A2).u();
                let result = os_call!(os, mem, |os| os.read(mem, fd, buf, count));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Sbrk => {
                let incr = reg_file.get(RegName::A0).i();
                let sp = reg_file.get(RegName::SP).addr();
                let result = os_call!(os, mem, |os| os.sbrk(sp, incr));
                reg_file.set(RegName::A0, result);
            }
            SyscallCode::Write => {
                let fd = reg_file.get(RegName::A0).i();
                let buf = reg_file.get(RegName::A1).addr();
                let count = reg_file.get(RegName::A2).u();
                let result = os_call!(os, mem, |os| os.write(mem, fd, buf, count));
                reg_file.set(RegName::A0, result);
            }
        }
    }

    fn get_exit_code(&self) -> ExitCode {
        self.regs.get(RegName::A0).i()
    }
}

impl Sequential for Cpu<'_> {
    fn finish_cycle(&mut self) {
        self.pc.finish_cycle();
        self.if_id_regs.finish_cycle();
        self.id_is_regs.finish_cycle();
        self.rob.finish_cycle();
        self.schedulers.finish_cycle();
        self.regs.finish_cycle();
        self.cdb.finish_cycle();
        self.load_queue.finish_cycle();
    }
}
