use crate::instr::decode::encoding::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::{AluSrcA, Instruction};

use cpu_sim_derive::Decode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Jalr(ITypeFormat);

impl From<Jalr> for Instruction {
    fn from(value: Jalr) -> Self {
        Instruction::Jump {
            base: AluSrcA::Reg(value.0.rs1()),
            offset: value.0.imm(),
            apply_mask: true,
            link_reg: value.0.rd(),
        }
    }
}

//#region Display

impl Jalr {
    const JR_DISPLAY_NAME: &'static str = "jr";
}

impl fmt::Display for Jalr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_jr = self.0.rd().is_zero() && self.0.imm() == 0;

        let name = if is_jr {
            Self::JR_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if !is_jr {
            self.0.rd().fmt(f)?;
            write!(f, ", ")?;
        }

        self.0.rs1().fmt(f)?;

        if !is_jr {
            write!(f, ", ")?;
            self.0.imm().fmt(f)?;
        }

        Ok(())
    }
}

//#endregion
