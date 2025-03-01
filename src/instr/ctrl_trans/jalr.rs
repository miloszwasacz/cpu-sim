use crate::components::cpu::alu::AluControl;
use crate::instr::decode::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::execute::{AluSrcA, AluSrcB, ExecUnit};
use crate::instr::issue::Branch;
use crate::instr::{Decode, Execute, Issue};

use cpu_sim_derive::{Decode, Instr, MemoryAccess, Writeback};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, MemoryAccess, Writeback, Instr)]
pub struct Jalr(ITypeFormat);

impl Issue for Jalr {
    fn branch(&self) -> Branch {
        Branch::Jump
    }
}

impl Execute for Jalr {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::Branch
    }

    fn alu_src_a(&self) -> AluSrcA {
        AluSrcA::Reg
    }

    fn alu_src_b(&self) -> AluSrcB {
        AluSrcB::Imm
    }

    fn alu_control(&self) -> AluControl {
        AluControl::Add
    }

    fn mask_jump_target(&self) -> bool {
        true
    }
}

//#region Display

impl Jalr {
    const JR_DISPLAY_NAME: &'static str = "jr";
}

impl fmt::Display for Jalr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_jr = self.rd().is_zero() && self.imm() == 0;

        let name = if is_jr {
            Self::JR_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if !is_jr {
            self.rd().fmt(f)?;
            write!(f, ", ")?;
        }

        self.rs1().fmt(f)?;

        if !is_jr {
            write!(f, ", ")?;
            self.imm().fmt(f)?;
        }

        Ok(())
    }
}

//#endregion
