pub use self::b_type::BTypeFormat;
pub use self::i_type::ITypeFormat;
pub use self::j_type::JTypeFormat;
pub use self::r_type::RTypeFormat;
pub use self::s_type::STypeFormat;
pub use self::u_type::UTypeFormat;
use crate::instr::{Decode, Writeback};

use std::fmt;

mod b_type;
mod i_type;
mod j_type;
mod r_type;
mod s_type;
mod u_type;

pub trait EncodingFormat: fmt::Debug + fmt::Display + Decode + Writeback {}

macro_rules! delegate_decode {
    () => {
        delegate::delegate! {
            to self.0 {
                fn rs1(&self) -> RegName;
                fn rs2(&self) -> RegName;
                fn rd(&self) -> RegName;
                fn imm(&self) -> Immediate;
            }
        }
    };
}
use delegate_decode;
