//! Pipeline regs
//TODO Better docs

use super::{RegData, RegName};
use crate::components::cpu::alu::AluControl;
use crate::components::cpu::circuit::{Circuit, ClockCycle};
use crate::components::cpu::error::{
    DecodeError, ExecuteError, FetchError, IssueError, MemAccessError, WritebackError,
};
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::execute::{AluSrcA, AluSrcB, ExecUnit};
use crate::instr::issue::Branch;
use crate::instr::mem_access::{MemRead, MemWrite};
use crate::instr::raw::RawInstrBits;
use crate::instr::{EnvTrap, Immediate, Instr};

use std::cell::Cell;
use std::rc::Rc;

//#region Pipeline regs

#[derive(Clone)]
pub struct PipelineRegs<T>(Rc<PipelineRegsInner<T>>);

struct PipelineRegsInner<T> {
    regs: Circuit<Cell<T>>,
    enabled: Cell<bool>,
    written: Cell<Option<ClockCycle>>,
    cleared: Cell<Option<ClockCycle>>,
}

impl<T: Default> PipelineRegs<T> {
    pub fn new() -> Self {
        Self(Rc::new(PipelineRegsInner {
            regs: Default::default(),
            enabled: Cell::new(true),
            written: Cell::new(None),
            cleared: Cell::new(None),
        }))
    }

    /// Sets the Enabled (`EN`) signal
    pub fn en(&mut self, en: bool) {
        self.0.enabled.set(en);
    }

    /// Sets the Clear (`CLR`) signal
    pub fn clr(&self, clr: bool, clock_half: ClockCycle) {
        if clr {
            self.0.cleared.set(Some(clock_half));
        }
    }

    pub fn read(&self, clock_half: ClockCycle) -> T
    where
        T: Copy,
    {
        match self.0.cleared.get() {
            Some(clr_clk) if clr_clk <= clock_half => T::default(),
            _ => self.0.regs.read_cell(clock_half),
        }
    }

    pub fn write(&mut self, clock_half: ClockCycle, value: T) {
        if !self.0.enabled.get() {
            return;
        }

        self.0.written.set(Some(clock_half));
        self.0.regs.write_cell(clock_half, value);
    }

    pub fn reset(&mut self) {
        self.0.regs.reset();
        match (self.0.written.get(), self.0.cleared.get()) {
            (_, None) => {}
            (Some(wrt), Some(clr)) if wrt > clr => {}
            _ => {
                self.write(ClockCycle::SecondHalf, T::default());
                self.0.regs.reset();
            }
        }
        self.0.enabled.set(true);
        self.0.written.set(None);
        self.0.cleared.set(None);
    }
}

impl<T: Default> Default for PipelineRegs<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FetchRegs {
    pub pc: Pc,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DecodeRegs {
    pub pc_ctrl: PcControl,
    pub err_ctrl: ErrorControl,
    pub instr: RawInstrBits,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IssueRegs {
    pub pc_ctrl: PcControl,
    pub is_ctrl: IssueControl,
    pub ex_ctrl: ExecuteControl,
    pub mem_ctrl: MemAccessControl,
    pub wb_ctrl: WritebackControl,
    pub err_ctrl: ErrorControl,
    pub rs1: RegName,
    pub rs2: RegName,
    pub rd: RegName,
    pub imm: Immediate,
}

impl IssueRegs {
    pub fn from_instr(instr: &dyn Instr, pc_ctrl: PcControl, err_ctrl: ErrorControl) -> Self {
        let is_ctrl = IssueControl {
            branch: instr.branch(),
        };
        let ex_ctrl = ExecuteControl {
            exec_unit: instr.exec_unit(),
            alu_src_a: instr.alu_src_a(),
            alu_src_b: instr.alu_src_b(),
            alu_control: instr.alu_control(),
            jump: false,
            mask_jump_target: instr.mask_jump_target(),
            env_trap: instr.env_trap(),
        };
        let mem_ctrl = MemAccessControl {
            mem_read: instr.mem_read(),
            mem_write: instr.mem_write(),
        };
        let wb_ctrl = WritebackControl {
            mem_to_reg: mem_ctrl.mem_read.is_some(),
            reg_write: instr.reg_write(),
        };
        let rs1 = instr.rs1();
        let rs2 = instr.rs2();
        let rd = instr.rd();
        let imm = instr.imm();

        Self {
            pc_ctrl,
            is_ctrl,
            ex_ctrl,
            mem_ctrl,
            wb_ctrl,
            err_ctrl,
            rs1,
            rs2,
            rd,
            imm,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteRegs {
    pub pc_ctrl: PcControl,
    pub ex_ctrl: ExecuteControl,
    pub mem_ctrl: MemAccessControl,
    pub wb_ctrl: WritebackControl,
    pub err_ctrl: ErrorControl,
    pub src1: (RegName, RegData),
    pub src2: (RegName, RegData),
    pub write_reg: RegName,
    pub imm: Immediate,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MemAccessRegs {
    pub mem_ctrl: MemAccessControl,
    pub wb_ctrl: WritebackControl,
    pub err_ctrl: ErrorControl,
    pub alu_out: RegData,
    pub write_data: RegData,
    pub write_reg: RegName,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WritebackRegs {
    pub wb_ctrl: WritebackControl,
    pub err_ctrl: ErrorControl,
    pub alu_out: RegData,
    pub read_data: RegData,
    pub write_reg: RegName,
}

//#endregion

//#region Control signals

#[derive(Debug, Clone, Copy, Default)]
pub struct PcControl {
    pub pc: Pc,
    pub pc_plus4: Pc,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IssueControl {
    pub branch: Branch,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteControl {
    pub exec_unit: ExecUnit,
    pub alu_src_a: AluSrcA,
    pub alu_src_b: AluSrcB,
    pub alu_control: AluControl,
    pub jump: bool,
    pub mask_jump_target: bool,
    pub env_trap: Option<EnvTrap>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MemAccessControl {
    pub mem_write: Option<MemWrite>,
    pub mem_read: Option<MemRead>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WritebackControl {
    pub mem_to_reg: bool,
    pub reg_write: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ErrorControl {
    pub pc: Pc,
    pub fetch_error: Option<FetchError>,
    pub decode_error: Option<DecodeError>,
    pub issue_error: Option<IssueError>,
    pub execute_error: Option<ExecuteError>,
    pub mem_access_error: Option<MemAccessError>,
    pub writeback_error: Option<WritebackError>,
    pub jump_to_self: bool,
}

pub type Jump = Option<Address>;

//#endregion
