use crate::components::cpu::reg::RegName;
use crate::components::cpu::AluControl;
use crate::instr::decode::encoding::UTypeFormat;
use crate::instr::{AluSrcA, AluSrcB, Instruction};

use cpu_sim_derive::{Decode, Display};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Decode)]
pub struct Lui(UTypeFormat);

impl From<Lui> for Instruction {
    fn from(value: Lui) -> Self {
        Instruction::Alu {
            ctrl: AluControl::Add,
            src1: AluSrcA::Reg(RegName::ZERO),
            src2: AluSrcB::Imm(value.0.imm()),
            dest: value.0.rd(),
        }
    }
}
