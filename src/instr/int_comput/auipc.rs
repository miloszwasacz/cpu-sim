use crate::components::cpu::alu::AluControl;
use crate::instr::decode::UTypeFormat;
use crate::instr::execute::{AluSrcA, AluSrcB, ExecUnit};
use crate::instr::Execute;

use cpu_sim_derive::{Decode, Display, Instr, Issue, MemoryAccess, Writeback};

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, Decode, Issue, MemoryAccess, Writeback, Instr,
)]
pub struct Auipc(UTypeFormat);

impl Execute for Auipc {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::Alu
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
