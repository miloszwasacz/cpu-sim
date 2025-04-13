use crate::instr::full::FullInstruction;
use crate::instr::raw::{RawInstr, RawInstrBits};

pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&mut self, instr: RawInstrBits) -> Result<FullInstruction, RawInstr> {
        RawInstr::new(instr).decode()
    }
}
