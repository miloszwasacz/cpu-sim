pub use self::encoding::*;
pub(crate) use self::shared::REG_LEN;
use super::raw::RawInstr;
use super::Immediate;
use crate::components::cpu::reg::RegName;

mod encoding;
mod generated;
mod shared;

pub trait Decode {
    fn decode(raw: RawInstr) -> Self
    where
        Self: Sized;

    fn rs1(&self) -> RegName {
        Default::default()
    }

    fn rs2(&self) -> RegName {
        Default::default()
    }

    fn rd(&self) -> RegName {
        Default::default()
    }

    fn imm(&self) -> Immediate {
        Default::default()
    }
}
