use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;

pub enum RawInstrType {
    Regular,
    Jump,
    Branch,
}

impl RawInstr {
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
