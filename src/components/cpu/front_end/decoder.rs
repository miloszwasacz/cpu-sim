use crate::components::cpu::error::{DecodeError, Exception};
use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;

#[derive(Clone)]
pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&mut self, instr: RawInstr) -> Result<FullInstruction, Exception> {
        instr
            .decode()
            .map_err(DecodeError::InvalidInstruction)
            .map_err(Into::into)
    }
}
