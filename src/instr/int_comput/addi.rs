use crate::components::cpu::AluControl;
use crate::instr::decode::encoding::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::{AluSrcA, AluSrcB, Instruction};

use cpu_sim_derive::Decode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Addi(ITypeFormat);

impl From<Addi> for Instruction {
    //noinspection DuplicatedCode
    fn from(value: Addi) -> Self {
        Instruction::Alu {
            ctrl: AluControl::Add,
            src1: AluSrcA::Reg(value.0.rs1()),
            src2: AluSrcB::Imm(value.0.imm()),
            dest: value.0.rd(),
        }
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

        let is_li = self.0.rs1().is_zero();
        let is_mv = self.0.imm() == 0;

        if self.0.rd().is_zero() && is_li && is_mv {
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
        self.0.rd().fmt(f)?;
        if !is_li {
            write!(f, ", ")?;
            self.0.rs1().fmt(f)?;
            if is_mv {
                return Ok(());
            }
        }
        write!(f, ", ")?;
        self.0.imm().fmt(f)
    }
}

//#endregion
