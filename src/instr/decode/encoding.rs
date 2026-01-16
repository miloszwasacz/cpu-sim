//! Encoding formats of RISC-V instructions.

pub use self::b_type::BTypeFormat;
pub use self::csr_type::CsrTypeFormat;
pub use self::csri_type::{CsrImmediate, CsriTypeFormat};
pub use self::i_type::ITypeFormat;
pub use self::j_type::JTypeFormat;
pub use self::r_type::RTypeFormat;
pub use self::s_type::STypeFormat;
pub use self::u_type::UTypeFormat;
use crate::instr::decode::Decode;

use std::fmt;

mod b_type;
mod csr_type;
mod csri_type;
mod i_type;
mod j_type;
mod r_type;
mod s_type;
mod u_type;

#[allow(unused)]
trait EncodingFormat: fmt::Debug + fmt::Display + Decode {}
