use crate::components::cpu::alu::AluControl;
use crate::instr::decode::JTypeFormat;
use crate::instr::display::display_width;
use crate::instr::execute::{AluSrcA, AluSrcB, ExecUnit};
use crate::instr::issue::Branch;
use crate::instr::{Decode, Execute, Issue};

use cpu_sim_derive::{Decode, Instr, MemoryAccess, Writeback};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, MemoryAccess, Writeback, Instr)]
pub struct Jal(JTypeFormat);

impl Issue for Jal {
    fn branch(&self) -> Branch {
        Branch::Jump
    }
}

impl Execute for Jal {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::Branch
    }

    fn alu_src_a(&self) -> AluSrcA {
        AluSrcA::Pc
    }

    fn alu_src_b(&self) -> AluSrcB {
        AluSrcB::Imm
    }

    fn alu_control(&self) -> AluControl {
        AluControl::Add
    }
}

//#region Display

impl Jal {
    const J_DISPLAY_NAME: &'static str = "j";
}

impl fmt::Display for Jal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_j = self.rd().is_zero();

        let name = if is_j {
            Self::J_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if is_j {
            self.imm().fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}

//#endregion
