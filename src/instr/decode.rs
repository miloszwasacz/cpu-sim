pub(crate) use self::shared::REG_LEN;
use crate::instr::raw::RawInstr;

pub(super) mod encoding;
mod generated;
mod shared;

pub trait Decode {
    fn decode(raw: RawInstr) -> Self
    where
        Self: Sized;
}
