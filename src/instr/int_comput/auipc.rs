use crate::components::cpu::AluControl;
use crate::instr::decode::encoding::UTypeFormat;
use crate::instr::{AluSrcA, AluSrcB, Instruction};

use cpu_sim_derive::{Decode, Display};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Auipc(UTypeFormat);

impl From<Auipc> for Instruction {
    fn from(value: Auipc) -> Self {
        Instruction::Alu {
            ctrl: AluControl::Add,
            src1: AluSrcA::Pc,
            src2: AluSrcB::Imm(value.0.imm()),
            dest: value.0.rd(),
        }
    }
}
