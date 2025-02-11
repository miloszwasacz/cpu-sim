use self::agu::Agu;
use self::alu::Alu;
use super::circuit::{Circuit, ClockCycle};
use super::error::{ExecuteError, WritebackError};
use super::front_end::DecodeRegs;
use super::reg::arf::{ArchRegFile, ArchRegName};
use super::reg::RegData;
use super::{make_pipeline_regs, PipelineRegs, ProgramCounter};
use crate::components::memory::Address;
use crate::instr::execute::ExecuteResult;
use crate::instr::Instr;

use std::rc::Rc;

pub mod agu;
pub mod alu;

pub(super) enum EnvTrap {
    None,
    Syscall(Address),
    Break(Address),
}

pub(super) struct ExecutionEngine {
    // Execute
    decode_regs: PipelineRegs<DecodeRegs>,
    int_reg_file: Circuit<ArchRegFile>,
    reg_file_read_regs: PipelineRegs<RegFileRegs>,
    alu: Circuit<Alu>,
    alu_regs: PipelineRegs<AluRegs>,
    load_agu: Circuit<Agu>,
    load_agu_regs: PipelineRegs<LoadAguRegs>,
    store_agu: Circuit<Agu>,
    store_agu_regs: PipelineRegs<StoreAguRegs>,

    // Writeback
    writeback_regs: PipelineRegs<WritebackRegs>,
    reg_file_write_regs: PipelineRegs<RegFileRegs>,
}

impl ExecutionEngine {
    pub(super) fn new(decode_regs: &PipelineRegs<DecodeRegs>) -> Self {
        let decode_regs = decode_regs.clone();
        let int_reg_file = ArchRegFile::new().into();
        let reg_file_read_regs = make_pipeline_regs();
        let alu = Alu::new().into();
        let alu_regs = make_pipeline_regs();
        let load_agu = Agu::new().into();
        let load_agu_regs = make_pipeline_regs();
        let store_agu = Agu::new().into();
        let store_agu_regs = make_pipeline_regs();

        let writeback_regs = make_pipeline_regs();
        let reg_file_write_regs = make_pipeline_regs();

        Self {
            decode_regs,
            int_reg_file,
            reg_file_read_regs,
            alu,
            alu_regs,
            load_agu,
            load_agu_regs,
            store_agu,
            store_agu_regs,

            writeback_regs,
            reg_file_write_regs,
        }
    }

    pub(super) unsafe fn int_reg_file(&mut self) -> &mut ArchRegFile {
        self.int_reg_file.inner_mut()
    }

    pub(super) fn id_ex_write_reg(&self) -> Option<ArchRegName> {
        unsafe {
            self.decode_regs
                .borrow()
                .inner()
                .instr
                .as_ref()
                .and_then(|instr| instr.write_reg())
        }
    }

    pub(super) fn mem_wb_write_reg(&self) -> Option<ArchRegName> {
        unsafe {
            self.writeback_regs
                .borrow()
                .inner()
                .instr
                .as_ref()
                .and_then(|instr| instr.write_reg())
        }
    }

    pub(super) fn alu_regs(&self) -> &PipelineRegs<AluRegs> {
        &self.alu_regs
    }

    pub(super) fn load_agu_regs(&self) -> &PipelineRegs<LoadAguRegs> {
        &self.load_agu_regs
    }

    pub(super) fn store_agu_regs(&self) -> &PipelineRegs<StoreAguRegs> {
        &self.store_agu_regs
    }

    pub(super) fn writeback_regs(&self) -> &PipelineRegs<WritebackRegs> {
        &self.writeback_regs
    }

    pub(super) fn reg_file_write_regs(&self) -> &PipelineRegs<RegFileRegs> {
        &self.reg_file_write_regs
    }

    pub fn execute(&mut self, pc: &mut ProgramCounter) -> Result<EnvTrap, ExecuteError> {
        let mut trap = EnvTrap::None;

        let reg_file = self
            .reg_file_read_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .0;

        let decode_regs_circ = self.decode_regs.borrow();
        let decode_regs = decode_regs_circ.read(ClockCycle::FirstHalf);
        let instr = decode_regs.instr.as_ref();
        let addr = decode_regs.addr;

        // We can mark all those units as 'in use' since they are not shared between different stages of the pipeline.

        let alu = self.alu.write(ClockCycle::Full);
        let mut alu_regs_circ = self.alu_regs.borrow_mut();
        let alu_regs = alu_regs_circ.write(ClockCycle::SecondHalf);
        *alu_regs = Default::default();

        let load_agu = self.load_agu.write(ClockCycle::Full);
        let mut load_agu_regs_circ = self.load_agu_regs.borrow_mut();
        let load_agu_regs = load_agu_regs_circ.write(ClockCycle::SecondHalf);
        *load_agu_regs = Default::default();

        let store_agu = self.store_agu.write(ClockCycle::Full);
        let mut store_agu_regs_circ = self.store_agu_regs.borrow_mut();
        let store_agu_regs = store_agu_regs_circ.write(ClockCycle::SecondHalf);
        *store_agu_regs = Default::default();

        if let Some(instr) = instr {
            // TODO Move printing to diagnostics
            println!("{:05x}    {:#}", addr, instr);

            match instr.execute(&reg_file, pc, alu, load_agu, store_agu)? {
                ExecuteResult::Alu(dest, data) => {
                    let instr = Some(instr.clone());
                    *alu_regs = AluRegs { instr, dest, data };
                }
                ExecuteResult::Branch => {}
                ExecuteResult::LoadAgu(dest, addr) => {
                    let instr = Some(instr.clone());
                    *load_agu_regs = LoadAguRegs { instr, dest, addr };
                }
                ExecuteResult::StoreAgu(addr, data) => {
                    let instr = Some(instr.clone());
                    *store_agu_regs = StoreAguRegs { instr, addr, data };
                }
                ExecuteResult::Ecall => trap = EnvTrap::Syscall(addr),
                ExecuteResult::Ebreak => trap = EnvTrap::Break(addr),
            }
        }

        self.reg_file_read_regs
            .borrow_mut()
            .write(ClockCycle::SecondHalf)
            .0 = *self.int_reg_file.read(ClockCycle::SecondHalf);

        Ok(trap)
    }

    pub fn writeback(&mut self) -> Result<(), WritebackError> {
        let write_regs = self.reg_file_write_regs.borrow_mut();
        let reg_file = self.int_reg_file.write(ClockCycle::FirstHalf);
        *reg_file = write_regs.read(ClockCycle::FirstHalf).0;

        Ok(())
    }

    pub fn finish_execute_cycle(&mut self) {
        self.decode_regs.borrow_mut().reset();
        self.int_reg_file.reset();
        self.reg_file_read_regs.borrow_mut().reset();
        self.alu.reset();
        self.alu_regs.borrow_mut().reset();
        self.load_agu.reset();
        self.load_agu_regs.borrow_mut().reset();
        self.store_agu.reset();
        self.store_agu_regs.borrow_mut().reset();
    }

    pub fn finish_writeback_cycle(&mut self) {
        self.reg_file_write_regs.borrow_mut().reset();
    }
}

//#region Pipeline registers

#[derive(Debug, Default)]
pub(super) struct RegFileRegs(pub ArchRegFile);

#[derive(Debug, Default)]
pub(super) struct AluRegs {
    pub instr: Option<Rc<dyn Instr>>,
    pub dest: ArchRegName,
    pub data: RegData,
}

#[derive(Debug, Default)]
pub(super) struct LoadAguRegs {
    pub instr: Option<Rc<dyn Instr>>,
    pub dest: ArchRegName,
    pub addr: Address,
}

#[derive(Debug, Default)]
pub(super) struct StoreAguRegs {
    pub instr: Option<Rc<dyn Instr>>,
    pub addr: Address,
    pub data: RegData,
}

#[derive(Debug, Default)]
pub(super) struct WritebackRegs {
    pub instr: Option<Rc<dyn Instr>>,
}

//#endregion
