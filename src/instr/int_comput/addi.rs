use crate::components::cpu::alu::AluControl;
use crate::instr::decode::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::execute::{AluSrcB, ExecUnit};
use crate::instr::{Decode, Execute};

use cpu_sim_derive::{Decode, Instr, Issue, MemoryAccess, Writeback};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Issue, MemoryAccess, Writeback, Instr)]
pub struct Addi(ITypeFormat);

impl Execute for Addi {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::Alu
    }

    fn alu_src_b(&self) -> AluSrcB {
        AluSrcB::Imm
    }

    fn alu_control(&self) -> AluControl {
        AluControl::Add
    }
}

//#region Display

impl Addi {
    const NOP_DISPLAY_NAME: &'static str = "nop";
    const LI_DISPLAY_NAME: &'static str = "li";
    const MV_DISPLAY_NAME: &'static str = "mv";
}

impl fmt::Display for Addi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);

        let is_li = self.rs1().is_zero();
        let is_mv = self.imm() == 0;

        if self.rd().is_zero() && is_li && is_mv {
            return write!(f, "{:<width$}", Self::NOP_DISPLAY_NAME);
        }

        let name = if is_li {
            Self::LI_DISPLAY_NAME
        } else if is_mv {
            Self::MV_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        self.rd().fmt(f)?;
        if !is_li {
            write!(f, ", ")?;
            self.rs1().fmt(f)?;
            if is_mv {
                return Ok(());
            }
        }
        write!(f, ", ")?;
        self.imm().fmt(f)
    }
}

//#endregion
