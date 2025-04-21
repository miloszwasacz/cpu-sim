use crate::components::cpu::error::{DecodeError, Exception};
use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;

#[derive(Clone)]
pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(
        &mut self,
        instr: Result<RawInstr, Exception>,
    ) -> Result<FullInstruction, Exception> {
        instr.and_then(|instr| {
            instr
                .decode()
                .map_err(|raw| DecodeError::InvalidInstruction(raw).into())
        })
    }
}
