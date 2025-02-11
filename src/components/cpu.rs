pub use self::exec_engine::agu::Agu;
pub use self::exec_engine::alu::Alu;
use self::exec_engine::ExecutionEngine;
use self::front_end::FrontEnd;
use self::mem_subsystem::MemorySubsystem;
use self::reg::{RegData, Register};
use super::memory::{Address, Memory};
use super::Bus;
use crate::instr::raw::RawInstrBits;
use std::cell::RefCell;

use crate::components::cpu::circuit::Circuit;
use std::error::Error;
use std::rc::Rc;

mod circuit;
pub mod error;
mod exec_engine;
mod front_end;
mod mem_subsystem;
pub mod reg;

type Result<E> = std::result::Result<(), E>;

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

    pub fn run(&mut self) -> Result<Box<dyn Error>> {
        loop {
            // The order is reversed to mimic parallelism

            // println!("Writeback");
            self.exec_engine.writeback().map_err(Box::new)?;
            self.exec_engine.finish_writeback_cycle();

            // println!("Memory Access");
            self.mem_subsystem.memory_access().map_err(Box::new)?;
            self.mem_subsystem.finish_memory_access_cycle();

            // println!("Execute");
            self.exec_engine.execute(&mut self.pc).map_err(Box::new)?;
            self.exec_engine.finish_execute_cycle();

            // println!("Decode");
            self.front_end.decode().map_err(Box::new)?;
            self.front_end.finish_decode_cycle();

            // println!("Fetch");
            self.front_end.fetch(&mut self.pc).map(Box::new)?;
            self.front_end.finish_fetch_cycle();

            // println!();
            self.pc.finish_cycle();
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
