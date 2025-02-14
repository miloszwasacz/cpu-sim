use self::agu::Agu;
use self::alu::Alu;
use super::circuit::{Circuit, ClockCycle};
use super::error::{ExecuteError, WritebackError};
use super::front_end::{DecodeOption, DecodeRegs};
use super::reg::arf::{ArchRegFile, ArchRegName};
use super::reg::RegData;
use super::{flush_pipeline_regs, make_pipeline_regs, PipelineRegs, ProgramCounter};
use crate::components::memory::Address;
use crate::instr::execute::ExecuteResult;
use crate::instr::Instr;

use std::rc::Rc;

pub mod agu;
pub mod alu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EnvTrap {
    None,
    Syscall(ProgramCounter),
    Break(ProgramCounter),
}

pub(super) struct ExecutionEngine {
    // Execute
    decode_regs: PipelineRegs<DecodeRegs>,
    int_reg_file: Circuit<ArchRegFile>,
    reg_file_read_regs: PipelineRegs<RegFileRegs>,
    alu: Circuit<Alu>,
    load_agu: Circuit<Agu>,
    store_agu: Circuit<Agu>,
    execute_regs: PipelineRegs<ExecuteRegs>,

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
        let load_agu = Agu::new().into();
        let store_agu = Agu::new().into();
        let execute_regs = make_pipeline_regs();

        let writeback_regs = make_pipeline_regs();
        let reg_file_write_regs = make_pipeline_regs();

        Self {
            decode_regs,
            int_reg_file,
            reg_file_read_regs,
            alu,
            load_agu,
            store_agu,
            execute_regs,

            writeback_regs,
            reg_file_write_regs,
        }
    }

    pub(super) unsafe fn int_reg_file(&mut self) -> &mut ArchRegFile {
        self.int_reg_file.inner_mut()
    }

    pub(super) fn id_ex_write_reg(&self) -> Option<ArchRegName> {
        self.decode_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .instr
            .as_ref()
            .into_option()
            .and_then(|instr| instr.write_reg())
    }

    pub(super) fn mem_wb_write_reg(&self) -> Option<ArchRegName> {
        self.writeback_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .instr
            .as_ref()
            .and_then(|instr| instr.write_reg())
    }

    pub(super) fn execute_regs(&self) -> &PipelineRegs<ExecuteRegs> {
        &self.execute_regs
    }

    pub(super) fn writeback_regs(&self) -> &PipelineRegs<WritebackRegs> {
        &self.writeback_regs
    }

    pub(super) fn reg_file_write_regs(&self) -> &PipelineRegs<RegFileRegs> {
        &self.reg_file_write_regs
    }

    pub fn execute(&mut self) -> Result<EnvTrap, ExecuteError> {
        let reg_file = self
            .reg_file_read_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .0;

        let decode_regs_circ = self.decode_regs.borrow();
        let decode_regs = decode_regs_circ.read(ClockCycle::FirstHalf);
        let instr = decode_regs.instr.as_ref();
        let pc = decode_regs.pc;

        // We can mark all those units as 'in use' since they are not shared between different stages of the pipeline.
        let alu = self.alu.write(ClockCycle::Full);
        let load_agu = self.load_agu.write(ClockCycle::Full);
        let store_agu = self.store_agu.write(ClockCycle::Full);

        let mut execute_regs_circ = self.execute_regs.borrow_mut();
        let execute_regs = execute_regs_circ.write(ClockCycle::SecondHalf);
        *execute_regs = ExecuteRegs {
            pc,
            instr: decode_regs.instr.clone(),
            ..Default::default()
        };

        self.reg_file_read_regs
            .borrow_mut()
            .write(ClockCycle::SecondHalf)
            .0 = *self.int_reg_file.read(ClockCycle::SecondHalf);

        if let DecodeOption::Some(instr) = instr {
            match instr.execute(&reg_file, pc, alu, load_agu, store_agu)? {
                ExecuteResult::Alu(dest, data) => {
                    execute_regs.alu = Some(AluRegs { dest, data });
                }
                ExecuteResult::Jump(target, dest, addr) => {
                    let data = addr as RegData;
                    execute_regs.alu = Some(AluRegs { dest, data });
                    execute_regs.branch = Some(target);
                    // TODO Can't we handle jumps earlier since we "know" the destination?
                }
                ExecuteResult::Branch(target) => {
                    execute_regs.branch = target;
                }
                ExecuteResult::LoadAgu(dest, addr) => {
                    execute_regs.load_agu = Some(LoadAguRegs { dest, addr });
                }
                ExecuteResult::StoreAgu(addr, data) => {
                    execute_regs.store_agu = Some(StoreAguRegs { addr, data });
                }
                ExecuteResult::Ecall => return Ok(EnvTrap::Syscall(pc)),
                ExecuteResult::Ebreak => return Ok(EnvTrap::Break(pc)),
            }
        }
        Ok(EnvTrap::None)
    }

    pub fn writeback(&mut self) -> Result<(), WritebackError> {
        // TODO Move logging to diagnostics
        let writeback_regs_circ = self.writeback_regs.borrow();
        let writeback_regs = writeback_regs_circ.read(ClockCycle::FirstHalf);
        if let Some(instr) = writeback_regs.instr.as_ref() {
            println!("{:05x}    {:#}", writeback_regs.pc.current.get(), instr);
        }

        let write_regs = self.reg_file_write_regs.borrow_mut();
        let reg_file = self.int_reg_file.write(ClockCycle::FirstHalf);
        *reg_file = write_regs.read(ClockCycle::FirstHalf).0;

        Ok(())
    }

    pub fn finish_execute_cycle(&mut self) {
        self.int_reg_file.reset();
        self.reg_file_read_regs.borrow_mut().reset();
        self.alu.reset();
        self.load_agu.reset();
        self.store_agu.reset();
        self.execute_regs.borrow_mut().reset();
    }

    pub fn finish_writeback_cycle(&mut self) {}

    pub fn flush(&mut self) {
        flush_pipeline_regs(&mut self.reg_file_read_regs);
        flush_pipeline_regs(&mut self.execute_regs);
    }
}

//#region Pipeline registers

#[derive(Debug, Default)]
pub(super) struct ExecuteRegs {
    pub pc: ProgramCounter,
    pub instr: DecodeOption<Rc<dyn Instr>>,
    pub alu: Option<AluRegs>,
    pub load_agu: Option<LoadAguRegs>,
    pub store_agu: Option<StoreAguRegs>,
    pub branch: Option<Address>,
}

#[derive(Debug, Default)]
pub(super) struct RegFileRegs(pub ArchRegFile);

#[derive(Debug, Default)]
pub(super) struct AluRegs {
    pub dest: ArchRegName,
    pub data: RegData,
}

#[derive(Debug, Default)]
pub(super) struct LoadAguRegs {
    pub dest: ArchRegName,
    pub addr: Address,
}

#[derive(Debug, Default)]
pub(super) struct StoreAguRegs {
    pub addr: Address,
    pub data: RegData,
}

#[derive(Debug, Default)]
pub(super) struct WritebackRegs {
    pub instr: Option<Rc<dyn Instr>>,
    pub pc: ProgramCounter,
}

//#endregion
