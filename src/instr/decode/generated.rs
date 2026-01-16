#![allow(unused_imports)]
use super::Decode;
use super::shared::*;
use crate::include_generated;
use crate::instr::{
    full::FullInstruction,
    raw::{RawInstr, RawInstrBits},
    *,
};

include_generated!("decode_impls.rs");
include_generated!("instr_impls.rs");

impl RawInstr {
    /// Attempts to decode a raw encoded instruction, returning the original encoding on failure.
    #[inline(always)]
    pub(crate) fn decode(self) -> Result<FullInstruction, Self> {
        decode(self)
    }
}
