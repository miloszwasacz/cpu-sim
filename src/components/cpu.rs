use self::error::Exception;
use self::exec_engine::ExecutionEngine;
pub use self::exec_engine::{agu, alu, branch};
use self::front_end::FrontEnd;
use self::hazard::HazardUnit;
use self::mem_subsystem::MemorySubsystem;
use self::reg::arf::ArchRegName;
use self::reg::RegFile;
use super::memory::{Address, Memory};
use super::Bus;
use crate::instr::{EnvTrap, SyscallCode};
use crate::os::Os;

use std::error::Error;

mod circuit;
pub mod error;
mod exec_engine;
mod front_end;
mod hazard;
mod mem_subsystem;
pub mod reg;

type Pc = Address;
pub type ExitCode = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuRun {
    Exit(ExitCode),
    Break,
}

enum TickResult {
    None,
    Trap(EnvTrap),
    Err(Box<dyn Error>),
    Halt,
}

// TODO Add diagnostics
pub struct Cpu<'m> {
    os: Os,
    front_end: FrontEnd<'m>,
    exec_engine: ExecutionEngine,
    mem_subsystem: MemorySubsystem<'m>,
    hazard_unit: HazardUnit,
    exit: bool,
}

impl<'m> Cpu<'m> {
    pub fn new(mem_bus: Bus<'m, Memory>) -> Self {
        let os = Os::new();
        let front_end = FrontEnd::new(mem_bus);
        let mut exec_engine = ExecutionEngine::new(front_end.issue_regs());
        let mem_subsystem = MemorySubsystem::new(exec_engine.mem_access_regs(), mem_bus);
        exec_engine.connect_writeback_regs(mem_subsystem.writeback_regs());
        let hazard_unit = HazardUnit::new(
            front_end.fetch_regs(),
            front_end.decode_regs(),
            front_end.issue_regs(),
            exec_engine.execute_regs(),
            exec_engine.mem_access_regs(),
            mem_subsystem.writeback_regs(),
        );
        let exit = false;

        Self {
            os,
            front_end,
            exec_engine,
            mem_subsystem,
            hazard_unit,
            exit,
        }
    }

    pub(crate) fn os(&mut self) -> &mut Os {
        &mut self.os
    }

    pub(crate) fn set_entrypoint(&mut self, entrypoint: Address) {
        unsafe {
            self.front_end.set_pc(entrypoint);
        }
    }

    pub fn run(&mut self) -> Result<CpuRun, Box<dyn Error>> {
        loop {
            match self.tick() {
                TickResult::None => {}
                TickResult::Trap(trap) => match trap {
                    EnvTrap::Syscall => self.handle_syscall(),
                    EnvTrap::Break => return Ok(CpuRun::Break),
                    EnvTrap::Exception(ex) => {
                        // TODO Exception handling
                        return Err(Box::new(ex));
                    }
                },
                TickResult::Err(err) => return Err(err),
                TickResult::Halt => {
                    let reg_file = unsafe { self.exec_engine.int_reg_file() };
                    let exit_code = reg_file.get(ArchRegName::A0).i();
                    return Ok(CpuRun::Exit(exit_code));
                }
            }
        }
    }

    fn tick(&mut self) -> TickResult {
        self.front_end.start_cycle();
        self.exec_engine.start_cycle();
        self.mem_subsystem.start_cycle();
        self.hazard_unit.start_cycle();

        // The order is reversed to allow in-place modification while mimicking parallelism

        // println!("Writeback");
        let err = self.exec_engine.writeback();

        // println!("Memory Access");
        self.mem_subsystem.memory_access(&mut self.hazard_unit);

        // println!("Execute")
        let (jump, env_trap) = self.exec_engine.execute(&mut self.hazard_unit);

        // println!("Issue")
        self.exec_engine.issue(&mut self.hazard_unit);

        // println!("Decode");
        self.front_end.decode();

        // println!("Fetch")
        self.front_end.fetch(jump);

        let halt = err.jump_to_self && self.exit;
        let result = err
            .try_into()
            .map(|ex: Option<Exception>| ex.map(Into::into).or(env_trap));
        match result {
            Ok(Some(trap)) => TickResult::Trap(trap),
            Ok(None) if halt => TickResult::Halt,
            Ok(None) => TickResult::None,
            Err(err) => TickResult::Err(err),
        }
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

        let reg_file = unsafe { self.exec_engine.int_reg_file() };
        let mem = unsafe { &mut self.mem_subsystem.mem() };
        // TODO Log unknown syscalls instead of panicking
        let syscall = SyscallCode::try_from(reg_file.get(ArchRegName::A7)).unwrap();
        match syscall {
            SyscallCode::Exit => self.exit = true,
            SyscallCode::Close => {
                let fd = reg_file.get(ArchRegName::A0).i();
                let result = os_call!(self.os, mem, |os| os.close(fd));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Fstat => {
                let fd = reg_file.get(ArchRegName::A0).i();
                let statbuf = reg_file.get(ArchRegName::A1).addr();
                let result = os_call!(self.os, mem, |os| os.fstat(mem, fd, statbuf));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Lseek => {
                let fd = reg_file.get(ArchRegName::A0).i();
                let offset = reg_file.get(ArchRegName::A1).i();
                let whence = reg_file.get(ArchRegName::A2).i();
                let result = os_call!(self.os, mem, |os| os.lseek(fd, offset, whence));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Read => {
                let fd = reg_file.get(ArchRegName::A0).i();
                let buf = reg_file.get(ArchRegName::A1).addr();
                let count = reg_file.get(ArchRegName::A2).u();
                let result = os_call!(self.os, mem, |os| os.read(mem, fd, buf, count));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Sbrk => {
                let incr = reg_file.get(ArchRegName::A0).i();
                let sp = reg_file.get(ArchRegName::SP).addr();
                let result = os_call!(self.os, mem, |os| os.sbrk(sp, incr));
                reg_file.set(ArchRegName::A0, result);
            }
            SyscallCode::Write => {
                let fd = reg_file.get(ArchRegName::A0).i();
                let buf = reg_file.get(ArchRegName::A1).addr();
                let count = reg_file.get(ArchRegName::A2).u();
                let result = os_call!(self.os, mem, |os| os.write(mem, fd, buf, count));
                reg_file.set(ArchRegName::A0, result);
            }
        }
    }
}
