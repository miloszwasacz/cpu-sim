use self::circuit::Circuit;
pub use self::exec_engine::agu::Agu;
pub use self::exec_engine::alu::Alu;
use self::exec_engine::{EnvTrap, ExecutionEngine};
use self::front_end::FrontEnd;
use self::mem_subsystem::MemorySubsystem;
use self::reg::arf::ArchRegName;
use self::reg::{RegData, RegFile, Register};
use super::memory::{Address, Memory};
use super::Bus;
use crate::instr::raw::RawInstrBits;
use crate::instr::SyscallCode;
use crate::os::Os;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod circuit;
pub mod error;
mod exec_engine;
mod front_end;
mod mem_subsystem;
pub mod reg;

type Stall = bool;
pub type ExitCode = i32;

pub enum CpuRun {
    Exit(ExitCode),
    Break,
}

// TODO Add diagnostics
pub struct Cpu<'m> {
    os: Os,
    pc: ProgramCounter,
    front_end: FrontEnd<'m>,
    exec_engine: ExecutionEngine,
    mem_subsystem: MemorySubsystem<'m>,
}

impl<'m> Cpu<'m> {
    pub fn new(mem_bus: Bus<'m, Memory>) -> Self {
        let os = Default::default();
        let pc = Default::default();
        let front_end = FrontEnd::new(mem_bus);
        let exec_engine = ExecutionEngine::new(front_end.decode_regs());
        let mem_subsystem = MemorySubsystem::new(
            exec_engine.execute_regs(),
            mem_bus,
            exec_engine.writeback_regs(),
            exec_engine.reg_file_write_regs(),
        );

        Self {
            os,
            pc,
            front_end,
            exec_engine,
            mem_subsystem,
        }
    }
    
    pub(crate) fn os(&mut self) -> &mut Os {
        &mut self.os
    }

    pub(crate) fn set_entrypoint(&mut self, entrypoint: Address) {
        self.pc.write(entrypoint);
        self.pc.finish_cycle();
    }

    pub fn run(&mut self) -> Result<CpuRun, Box<dyn Error>> {
        loop {
            let trap = self.tick()?;
            match trap {
                EnvTrap::None => {}
                EnvTrap::Syscall(pc) => {
                    if let Some(exit_code) = self.handle_syscall() {
                        self.save_pc(pc);
                        return Ok(CpuRun::Exit(exit_code));
                    }
                }
                EnvTrap::Break(pc) => {
                    self.save_pc(pc);
                    return Ok(CpuRun::Break);
                }
            }
        }
    }

    fn tick(&mut self) -> Result<EnvTrap, Box<dyn Error>> {
        // The order is reversed to mimic parallelism

        // println!("Writeback");
        let mem_wb_write_reg = self.exec_engine.mem_wb_write_reg();
        self.exec_engine.writeback().map_err(Box::new)?;

        // println!("Memory Access");
        let ex_mem_write_reg = self.mem_subsystem.ex_mem_write_reg();
        let jump_target = self.mem_subsystem.memory_access()?;

        // println!("Execute");
        let id_ex_write_reg = self.exec_engine.id_ex_write_reg();
        let trap = self.exec_engine.execute().map_err(Box::new)?;

        // println!("Decode");
        let stall = self
            .front_end
            .decode(id_ex_write_reg, ex_mem_write_reg, mem_wb_write_reg)
            .map_err(Box::new)?;

        // println!("Fetch");
        self.front_end.fetch(&mut self.pc, stall).map(Box::new)?;

        // println!();
        self.exec_engine.finish_writeback_cycle();
        self.mem_subsystem.finish_memory_access_cycle();
        self.exec_engine.finish_execute_cycle();
        self.front_end.finish_decode_cycle();
        self.front_end.finish_fetch_cycle();

        if let Some(jump_target) = jump_target {
            self.pc.write(jump_target);
            self.exec_engine.flush();
            self.front_end.flush();
        }

        self.pc.finish_cycle();
        Ok(trap)
    }

    fn save_pc(&mut self, pc: ProgramCounter) {
        self.pc = pc;
        self.pc.advance();
        self.pc.finish_cycle();
    }

    fn handle_syscall(&mut self) -> Option<ExitCode> {
        macro_rules! os_call {
            ($os:expr, $mem:expr, |os| os.$( $call:tt )+) => {
                match $os.$( $call )+ {
                    Ok(r) => r,
                    Err((r, errno)) => {
                        $os.set_errno($mem, errno);
                        r
                    }
                }
            };
        }
        
        let reg_file = unsafe { self.exec_engine.int_reg_file() };
        let mem = unsafe { &mut self.mem_subsystem.mem() };
        // TODO Log unknown syscalls instead of panicking
        let syscall = SyscallCode::try_from(reg_file.get(ArchRegName::SYSCALL_CODE).get()).unwrap();
        match syscall {
            // TODO Refactor exiting -- on ECALL, mark as ready to exit, and stop only when there is a jump to itself
            SyscallCode::Exit => return Some(reg_file.get(ArchRegName::A0).get()),
            SyscallCode::Close => {
                let fd = reg_file.get(ArchRegName::A0).get();
                let result = os_call!(self.os, mem, |os| os.close(fd));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Lseek => {
                let fd = reg_file.get(ArchRegName::A0).get();
                let offset = reg_file.get(ArchRegName::A1).get();
                let whence = reg_file.get(ArchRegName::A2).get();
                let result = os_call!(self.os, mem, |os| os.lseek(fd, offset, whence));
                reg_file.set(ArchRegName::A2, result);
            }
            SyscallCode::Read => {
                let fd = reg_file.get(ArchRegName::A0).get();
                let buf = reg_file.get(ArchRegName::A1).get() as Address;
                let count = reg_file.get(ArchRegName::A2).get_unsigned();
                let result = os_call!(self.os, mem, |os| os.read(mem, fd, buf, count));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Sbrk => {
                let incr = reg_file.get(ArchRegName::A0).get();
                let result = os_call!(self.os, mem, |os| os.sbrk(incr));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Write => {
                let fd = reg_file.get(ArchRegName::A0).get();
                let buf = reg_file.get(ArchRegName::A1).get() as Address;
                let count = reg_file.get(ArchRegName::A2).get_unsigned();
                let result = os_call!(self.os, mem, |os| os.write(mem, fd, buf, count));
                reg_file.set(ArchRegName::A0, result);
            }
        }
        None
    }
}

//#region PC

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProgramCounter {
    current: Register,
    next: Register,
}

impl ProgramCounter {
    pub fn read(&self) -> Address {
        self.current.get() as Address
    }

    pub fn read_next(&self) -> Address {
        self.next.get() as Address
    }

    fn advance(&mut self) {
        let current = self.current.get();
        let next = current + size_of::<RawInstrBits>() as RegData;
        self.next.set(next);
    }

    pub fn write(&mut self, addr: Address) {
        self.next.set(addr as RegData);
    }

    fn finish_cycle(&mut self) {
        self.current = self.next;
    }
}

//#endregion

//#region Pipeline registers

type PipelineRegs<T> = Rc<RefCell<Circuit<T>>>;

fn make_pipeline_regs<T: Default>() -> PipelineRegs<T> {
    Rc::new(RefCell::new(Circuit::new(Default::default())))
}

fn flush_pipeline_regs<T: Default>(regs: &mut PipelineRegs<T>) {
    unsafe { *regs.borrow_mut().inner_mut() = Default::default() }
}

//#endregion
