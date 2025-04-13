#[allow(unused_imports)]
use super::shared::*;
#[allow(unused_imports)]
use super::Decode;
use crate::include_generated;
use crate::instr::raw::RawInstr;
#[allow(unused_imports)]
use crate::instr::raw::RawInstrBits;
#[allow(unused_imports)]
use crate::instr::*;
#[allow(unused_imports)]
use crate::instr::full::FullInstruction;

include_generated!("decode_impls.rs");
include_generated!("instr_impls.rs");

impl RawInstr {
    #[inline(always)]
    pub(crate) fn decode(self) -> Result<FullInstruction, Self> {
        decode(self)
    }
}
