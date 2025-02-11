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

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod circuit;
pub mod error;
mod exec_engine;
mod front_end;
mod mem_subsystem;
pub mod reg;

pub type ExitCode = i32;

pub enum CpuRun {
    Exit(ExitCode),
    Break,
}

// TODO Add diagnostics
pub struct Cpu<'m> {
    pc: ProgramCounter,
    front_end: FrontEnd<'m>,
    exec_engine: ExecutionEngine,
    mem_subsystem: MemorySubsystem<'m>,
}

impl<'m> Cpu<'m> {
    pub fn new(mem_bus: Bus<'m, Memory>) -> Self {
        let pc = Default::default();
        let front_end = FrontEnd::new(mem_bus);
        let exec_engine = ExecutionEngine::new(front_end.decode_regs());
        let mem_subsystem = MemorySubsystem::new(
            exec_engine.alu_regs(),
            exec_engine.load_agu_regs(),
            exec_engine.store_agu_regs(),
            mem_bus,
            exec_engine.reg_file_write_regs(),
        );

        Self {
            pc,
            front_end,
            exec_engine,
            mem_subsystem,
        }
    }

    pub(crate) fn set_entrypoint(&mut self, entrypoint: Address) {
        self.pc.write(entrypoint);
        self.pc.finish_cycle();
    }

    pub fn run(&mut self) -> Result<CpuRun, Box<dyn Error>> {
        loop {
            // The order is reversed to mimic parallelism

            // println!("Writeback");
            self.exec_engine.writeback().map_err(Box::new)?;
            self.exec_engine.finish_writeback_cycle();

            // println!("Memory Access");
            self.mem_subsystem.memory_access().map_err(Box::new)?;
            self.mem_subsystem.finish_memory_access_cycle();

            // println!("Execute");
            let trap = self.exec_engine.execute(&mut self.pc).map_err(Box::new)?;
            self.exec_engine.finish_execute_cycle();

            // println!("Decode");
            self.front_end.decode().map_err(Box::new)?;
            self.front_end.finish_decode_cycle();

            // println!("Fetch");
            self.front_end.fetch(&mut self.pc).map(Box::new)?;
            self.front_end.finish_fetch_cycle();

            // println!();

            match trap {
                EnvTrap::None => self.pc.finish_cycle(),
                EnvTrap::Syscall(pc) => {
                    let exit_code = self.handle_syscall();
                    self.save_pc(pc);
                    if let Some(exit_code) = exit_code {
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

    // pub fn tick(&mut self) -> Result<Box<dyn Error>> {
    //     // The order is reversed to mimic parallelism
    //     self.exec_engine.writeback();
    //     self.mem_subsystem.memory_access().map_err(Box::new)?;
    //     self.exec_engine.execute().map_err(Box::new)?;
    //     self.front_end.decode().map_err(Box::new)?;
    //     self.front_end.fetch(&mut self.pc).map(Box::new)?;
    //
    //     Ok(())
    // }

    fn save_pc(&mut self, pc: Address) {
        self.pc.write(pc);
        self.pc.advance();
        self.pc.finish_cycle();
    }

    fn handle_syscall(&mut self) -> Option<ExitCode> {
        let reg_file = unsafe { self.exec_engine.int_reg_file() };
        let syscall = SyscallCode::try_from(reg_file.get(ArchRegName::SYSCALL_CODE).get()).unwrap();
        match syscall {
            SyscallCode::Exit => Some(reg_file.get(ArchRegName::A0).get()),
        }
    }
}

//#region PC

#[derive(Debug, Default)]
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

//#endregion
