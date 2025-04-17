use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;

pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&mut self, instr: RawInstr) -> Result<FullInstruction, RawInstr> {
        instr.decode()
    }
}
