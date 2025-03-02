use super::circuit::{Circuit, ClockCycle};
use super::hazard::HazardUnit;
use super::reg::pipeline::{MemAccessControl, MemAccessRegs, PipelineRegs, WritebackRegs};
use crate::components::memory::Memory;
use crate::components::Bus;

use std::cell::RefMut;

pub struct MemorySubsystem<'m> {
    mem_access_regs: PipelineRegs<MemAccessRegs>,
    mem_bus: Circuit<Bus<'m, Memory>>,
    writeback_regs: PipelineRegs<WritebackRegs>,
}

impl<'m> MemorySubsystem<'m> {
    pub(super) fn new(
        mem_access_regs: &PipelineRegs<MemAccessRegs>,
        mem_bus: Bus<'m, Memory>,
    ) -> MemorySubsystem<'m> {
        let mem_access_regs = mem_access_regs.clone();
        let mem_bus = mem_bus.into();
        let writeback_regs = Default::default();

        Self {
            mem_access_regs,
            mem_bus,
            writeback_regs,
        }
    }

    pub fn writeback_regs(&self) -> &PipelineRegs<WritebackRegs> {
        &self.writeback_regs
    }

    pub(super) unsafe fn mem(&mut self) -> RefMut<'_, Memory> {
        self.mem_bus.inner_mut().borrow_mut()
    }

    pub fn start_cycle(&mut self) {
        self.mem_bus.reset();
        self.writeback_regs.reset();
    }

    pub fn memory_access(&mut self, hazard_unit: &mut HazardUnit) {
        let MemAccessRegs {
            mem_ctrl,
            wb_ctrl,
            err_ctrl,
            alu_out,
            write_data,
            write_reg,
        } = self.mem_access_regs.read(ClockCycle::FirstHalf);
        let MemAccessControl {
            mem_write,
            mem_read,
        } = mem_ctrl;

        {
            let mut mem = self.mem_bus.write(ClockCycle::FirstHalf).borrow_mut();
            if let Some(mem_write) = mem_write {
                mem_write(&mut mem, alu_out.addr(), write_data)
            }
        }
        let read_data = {
            let mem = self.mem_bus.read(ClockCycle::SecondHalf).borrow();
            match mem_read {
                Some(mem_read) => mem_read(&mem, alu_out.addr()),
                None => Default::default(),
            }
        };

        let writeback_regs = WritebackRegs {
            wb_ctrl,
            err_ctrl,
            alu_out,
            read_data,
            write_reg,
        };

        hazard_unit.mem_access_forward_out(writeback_regs);
        self.writeback_regs
            .write(ClockCycle::SecondHalf, writeback_regs);
    }
}
