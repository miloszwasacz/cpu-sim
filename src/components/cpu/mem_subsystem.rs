use super::circuit::{Circuit, ClockCycle};
use super::error::MemAccessError;
use super::exec_engine::{AluRegs, LoadAguRegs, RegFileRegs, StoreAguRegs, WritebackRegs};
use super::reg::arf::ArchRegName;
use super::reg::RegFile;
use super::PipelineRegs;
use crate::components::memory::Memory;
use crate::components::Bus;

pub struct MemorySubsystem<'m> {
    alu_regs: PipelineRegs<AluRegs>,
    load_agu_regs: PipelineRegs<LoadAguRegs>,
    store_agu_regs: PipelineRegs<StoreAguRegs>,
    mem_bus: Circuit<Bus<'m, Memory>>,
    writeback_regs: PipelineRegs<WritebackRegs>,
    reg_file_write_regs: PipelineRegs<RegFileRegs>,
}

impl<'m> MemorySubsystem<'m> {
    pub(super) fn new(
        alu_regs: &PipelineRegs<AluRegs>,
        load_agu_regs: &PipelineRegs<LoadAguRegs>,
        store_agu_regs: &PipelineRegs<StoreAguRegs>,
        mem_bus: Bus<'m, Memory>,
        writeback_regs: &PipelineRegs<WritebackRegs>,
        reg_file_write_regs: &PipelineRegs<RegFileRegs>,
    ) -> Self {
        let alu_regs = alu_regs.clone();
        let load_agu_regs = load_agu_regs.clone();
        let store_agu_regs = store_agu_regs.clone();
        let mem_bus = mem_bus.into();
        let writeback_regs = writeback_regs.clone();
        let reg_file_write_regs = reg_file_write_regs.clone();

        Self {
            alu_regs,
            load_agu_regs,
            store_agu_regs,
            mem_bus,
            writeback_regs,
            reg_file_write_regs,
        }
    }

    pub(super) fn ex_mem_write_reg(&self) -> Option<ArchRegName> {
        unsafe {
            self.alu_regs
                .borrow()
                .inner()
                .instr
                .as_ref()
                .and_then(|instr| instr.write_reg())
                .or_else(|| {
                    self.load_agu_regs
                        .borrow()
                        .inner()
                        .instr
                        .as_ref()
                        .and_then(|instr| instr.write_reg())
                })
                .or_else(|| {
                    self.store_agu_regs
                        .borrow()
                        .inner()
                        .instr
                        .as_ref()
                        .and_then(|instr| instr.write_reg())
                })
        }
    }

    pub fn memory_access(&mut self) -> Result<(), MemAccessError> {
        let alu_regs = self.alu_regs.borrow();
        let alu = alu_regs.read(ClockCycle::FirstHalf);

        let load_agu_regs = self.load_agu_regs.borrow();
        let load_agu = load_agu_regs.read(ClockCycle::FirstHalf);

        let store_agu_regs = self.store_agu_regs.borrow();
        let store_agu = store_agu_regs.read(ClockCycle::FirstHalf);

        let mut writeback_regs_circ = self.writeback_regs.borrow_mut();
        let writeback_regs = writeback_regs_circ.write(ClockCycle::SecondHalf);

        let mut reg_file_regs_circ = self.reg_file_write_regs.borrow_mut();
        let reg_file_regs = &mut reg_file_regs_circ.write(ClockCycle::SecondHalf).0;

        writeback_regs.instr = if let Some(instr) = &alu.instr {
            reg_file_regs.set(alu.dest, alu.data);
            Some(instr.clone())
        } else if let Some(instr) = &load_agu.instr {
            let mem = self.mem_bus.read(ClockCycle::Full).borrow();
            let data = instr.load(&mem, load_agu.addr)?;
            reg_file_regs.set(load_agu.dest, data);
            Some(instr.clone())
        } else if let Some(instr) = &store_agu.instr {
            let mut mem = self.mem_bus.write(ClockCycle::FirstHalf).borrow_mut();
            instr.store(&mut mem, store_agu.addr, store_agu.data)?;
            Some(instr.clone())
        } else {
            None
        };

        Ok(())
    }

    pub fn finish_memory_access_cycle(&mut self) {
        self.alu_regs.borrow_mut().reset();
        self.load_agu_regs.borrow_mut().reset();
        self.store_agu_regs.borrow_mut().reset();
        self.mem_bus.reset();
        self.writeback_regs.borrow_mut().reset();
        self.reg_file_write_regs.borrow_mut().reset();
    }
}
