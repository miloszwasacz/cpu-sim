//! Instruction decoding logic and utilities.

pub(crate) use self::shared::REG_LEN;
use crate::instr::raw::RawInstr;

pub(super) mod encoding;
mod generated;
mod shared;

/// A trait for decoding raw bits into more usable structures.
pub trait Decode {
    /// Decodes a raw instruction into a more usable representation. 
    fn decode(raw: RawInstr) -> Self
    where
        Self: Sized;
}
