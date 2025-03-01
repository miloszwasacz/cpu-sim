#[allow(unused_imports)]
use super::shared::*;
use crate::include_generated;
use crate::instr::raw::RawInstr;
#[allow(unused_imports)]
use crate::instr::raw::RawInstrBits;
#[allow(unused_imports)]
use crate::instr::*;

include_generated!("decode_impls.rs");

impl RawInstr {
    pub(crate) fn decode(self) -> Result<Box<dyn Instr>, Self> {
        decode(self)
    }
}
