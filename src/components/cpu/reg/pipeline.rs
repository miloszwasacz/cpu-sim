//! Pipeline registers
//TODO Improve docs

use crate::components::cpu::error::Exception;
use crate::components::cpu::front_end::PcPlus4;
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;
use crate::instr::Instruction;

#[derive(Debug, Clone, Copy)]
pub struct IfIdRegs {
    pub instr: Result<RawInstr, Exception>,
    pub pc: Pc,
    pub pc_plus_4: PcPlus4,
}

#[derive(Debug, Clone, Copy)]
pub struct IdIsRegs {
    pub instr: Result<(Instruction, FullInstruction), Exception>,
    pub pc: Pc,
    pub pc_plus_4: PcPlus4,
    /// The predicted address of a jump/branch.
    /// For PC-based jumps this is the actual target address.
    pub predicted: Option<Address>,
    /// The pre-computed target address of a branch.
    pub target: Option<Address>,
}
