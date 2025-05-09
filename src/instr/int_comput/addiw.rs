use crate::components::cpu::AluControl;
use crate::instr::decode::encoding::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::{AluSrcA, AluSrcB, Instruction};

use cpu_sim_derive::Decode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Addiw(ITypeFormat);

impl From<Addiw> for Instruction {
    //noinspection DuplicatedCode
    fn from(value: Addiw) -> Self {
        Instruction::Alu {
            ctrl: AluControl::Addw,
            src1: AluSrcA::Reg(value.0.rs1()),
            src2: AluSrcB::Imm(value.0.imm()),
            dest: value.0.rd(),
        }
    }
}

//#region Display

impl Addiw {
    const SEXT_W_DISPLAY_NAME: &'static str = "sext.w";
}

impl fmt::Display for Addiw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_sextw = self.0.imm() == 0;

        let name = if is_sextw {
            Self::SEXT_W_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        self.0.rd().fmt(f)?;
        write!(f, ", ")?;
        self.0.rs1().fmt(f)
    }
}

//#endregion
