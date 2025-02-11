use crate::components::cpu::error::DecodeError;
use crate::instr::raw::RawInstr;
use crate::instr::Instr;

use std::rc::Rc;

pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&self, instr: RawInstr) -> Result<Rc<dyn Instr>, DecodeError> {
        instr.decode().map_err(DecodeError::InvalidInstruction)
    }
}
