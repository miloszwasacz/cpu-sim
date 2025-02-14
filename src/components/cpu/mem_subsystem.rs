use super::circuit::{Circuit, ClockCycle};
use super::error::DecodeError;
use super::exec_engine::{ExecuteRegs, RegFileRegs, WritebackRegs};
use super::front_end::DecodeOption;
use super::reg::arf::ArchRegName;
use super::reg::RegFile;
use super::PipelineRegs;
use crate::components::memory::{Address, Memory};
use crate::components::Bus;

use std::cell::RefMut;
use std::error::Error;

pub struct MemorySubsystem<'m> {
    execute_regs: PipelineRegs<ExecuteRegs>,
    mem_bus: Circuit<Bus<'m, Memory>>,
    writeback_regs: PipelineRegs<WritebackRegs>,
    reg_file_write_regs: PipelineRegs<RegFileRegs>,
}

impl<'m> MemorySubsystem<'m> {
    pub(super) fn new(
        execute_regs: &PipelineRegs<ExecuteRegs>,
        mem_bus: Bus<'m, Memory>,
        writeback_regs: &PipelineRegs<WritebackRegs>,
        reg_file_write_regs: &PipelineRegs<RegFileRegs>,
    ) -> Self {
        let execute_regs = execute_regs.clone();
        let mem_bus = mem_bus.into();
        let writeback_regs = writeback_regs.clone();
        let reg_file_write_regs = reg_file_write_regs.clone();

        Self {
            execute_regs,
            mem_bus,
            writeback_regs,
            reg_file_write_regs,
        }
    }

    pub(super) unsafe fn mem(&mut self) -> RefMut<'_, Memory> {
        self.mem_bus.inner_mut().borrow_mut()
    }

    pub(super) fn ex_mem_write_reg(&self) -> Option<ArchRegName> {
        self.execute_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .instr
            .as_ref()
            .into_option()
            .and_then(|instr| instr.write_reg())
    }

    pub fn memory_access(&mut self) -> Result<Option<Address>, Box<dyn Error>> {
        let execute_regs_circ = self.execute_regs.borrow();
        let execute_regs = execute_regs_circ.read(ClockCycle::FirstHalf);

        let mut writeback_regs_circ = self.writeback_regs.borrow_mut();
        let writeback_regs = writeback_regs_circ.write(ClockCycle::SecondHalf);

        let mut reg_file_regs_circ = self.reg_file_write_regs.borrow_mut();
        let reg_file_regs = &mut reg_file_regs_circ.write(ClockCycle::SecondHalf).0;

        *writeback_regs = WritebackRegs {
            instr: execute_regs.instr.clone().into_option(),
            pc: execute_regs.pc,
        };
        match execute_regs.instr.as_ref() {
            DecodeOption::Some(instr) => {
                if let Some(alu) = &execute_regs.alu {
                    reg_file_regs.set(alu.dest, alu.data);
                }
                if let Some(load_agu) = &execute_regs.load_agu {
                    let mem = self.mem_bus.read(ClockCycle::Full).borrow();
                    let data = instr.load(&mem, load_agu.addr)?;
                    reg_file_regs.set(load_agu.dest, data);
                }
                if let Some(store_agu) = &execute_regs.store_agu {
                    let mut mem = self.mem_bus.write(ClockCycle::FirstHalf).borrow_mut();
                    instr.store(&mut mem, store_agu.addr, store_agu.data)?;
                }
            }
            DecodeOption::InvalidInstr(instr) => {
                return Err(Box::new(DecodeError::InvalidInstruction(instr)));
            }
            DecodeOption::None => {}
        };

        Ok(execute_regs.branch)
    }

    pub fn finish_memory_access_cycle(&mut self) {
        self.mem_bus.reset();
        self.writeback_regs.borrow_mut().reset();
        self.reg_file_write_regs.borrow_mut().reset();
    }
}
