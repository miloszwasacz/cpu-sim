use crate::instr::decode::encoding::JTypeFormat;
use crate::instr::display::display_width;
use crate::instr::{AluSrcA, Instruction};

use cpu_sim_derive::Decode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Jal(JTypeFormat);

impl From<Jal> for Instruction {
    fn from(value: Jal) -> Self {
        Instruction::Jump {
            base: AluSrcA::Pc,
            offset: value.0.imm(),
            apply_mask: false,
            link_reg: value.0.rd(),
        }
    }
}

//#region Display

impl Jal {
    const J_DISPLAY_NAME: &'static str = "j";
}

impl fmt::Display for Jal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_j = self.0.rd().is_zero();

        let name = if is_j {
            Self::J_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if is_j {
            self.0.imm().fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}

//#endregion
