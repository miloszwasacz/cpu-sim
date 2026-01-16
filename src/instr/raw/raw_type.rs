use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;

/// The type of instruction that could be easily inferred just from the OPCODE.
///
/// Used for branch predictors to distinguish between jumps and regular instructions.
pub enum RawInstrType {
    /// Regular instruction (does not modify the PC).
    Regular,
    /// Unconditional jump.
    Jump,
    /// Conditional jump (branch).
    Branch,
}

impl RawInstr {
    /// Returns what type of instruction this raw encoding represents.
    pub fn type_from_opcode(&self) -> RawInstrType {
        // The instruction wouldn't actually be decoded here; this is just for
        // convenience of not reimplementing extraction of opcode-specific bits.
        match self.decode() {
            Ok(instr) => match instr {
                FullInstruction::Jal(_) | FullInstruction::Jalr(_) => RawInstrType::Jump,
                FullInstruction::Beq(_)
                | FullInstruction::Bne(_)
                | FullInstruction::Blt(_)
                | FullInstruction::Bge(_)
                | FullInstruction::Bltu(_)
                | FullInstruction::Bgeu(_) => RawInstrType::Branch,
                _ => RawInstrType::Regular,
            },
            Err(_) => RawInstrType::Regular,
        }
    }
}
