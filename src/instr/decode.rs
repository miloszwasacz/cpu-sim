pub use self::b_type::BTypeFormat;
pub use self::i_type::ITypeFormat;
pub use self::j_type::JTypeFormat;
pub use self::r_type::RTypeFormat;
pub use self::s_type::STypeFormat;
pub use self::u_type::UTypeFormat;

pub(super) use self::generated::decode;
pub(crate) use self::shared::REG_LEN;
use super::stall::StallControl;
use crate::instr::raw::RawInstr;
use crate::instr::Instr;

mod b_type;
mod i_type;
mod j_type;
mod r_type;
mod s_type;
mod u_type;

pub trait Decode {
    fn decode(instr: RawInstr) -> Self
    where
        Self: Sized;
}

trait EncodingFormat: StallControl {
    fn decode(instr: RawInstr) -> Self;
}

mod shared {
    use crate::components::cpu::reg::arf::ArchRegName;
    use crate::instr::raw::{Bits, RawInstr};

    pub(crate) const REG_LEN: u64 = 5;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Opcode(Bits<{ Opcode::LEN }>);
    impl Opcode {
        const LEN: u64 = 7;
        const MSB: u64 = 6;

        pub const fn new(value: u64) -> Self {
            Self(Bits::new(value))
        }
    }

    pub(crate) const RD_LEN: u64 = REG_LEN;
    pub(crate) const RD_MSB: u64 = 11;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Funct3(Bits<{ Funct3::LEN }>);
    impl Funct3 {
        const LEN: u64 = 3;
        const MSB: u64 = 14;

        pub const fn new(value: u64) -> Self {
            Self(Bits::new(value))
        }
    }

    pub(crate) const RS1_LEN: u64 = REG_LEN;
    pub(crate) const RS1_MSB: u64 = 19;

    pub(crate) const RS2_LEN: u64 = REG_LEN;
    pub(crate) const RS2_MSB: u64 = 24;

    pub(crate) fn decode_opcode(instr: RawInstr) -> Opcode {
        Opcode(instr.extract_bits::<{ Opcode::LEN }, { Opcode::MSB }>())
    }

    pub(crate) fn decode_rd(instr: RawInstr) -> ArchRegName {
        instr.extract_bits::<RD_LEN, RD_MSB>().into()
    }

    pub(crate) fn decode_funct3(instr: RawInstr) -> Funct3 {
        Funct3(instr.extract_bits::<{ Funct3::LEN }, { Funct3::MSB }>())
    }

    pub(crate) fn decode_rs1(instr: RawInstr) -> ArchRegName {
        instr.extract_bits::<RS1_LEN, RS1_MSB>().into()
    }

    pub(crate) fn decode_rs2(instr: RawInstr) -> ArchRegName {
        instr.extract_bits::<RS2_LEN, RS2_MSB>().into()
    }
}

#[allow(dead_code)]
mod generated {
    #[allow(unused_imports)]
    use super::b_type::BType;
    #[allow(unused_imports)]
    use super::i_type::{decode_shift_type, IType, ShiftType};
    #[allow(unused_imports)]
    use super::j_type::JType;
    #[allow(unused_imports)]
    use super::r_type::{decode_funct7, Funct7, RType};
    #[allow(unused_imports)]
    use super::s_type::SType;
    #[allow(unused_imports)]
    use super::shared::*;
    #[allow(unused_imports)]
    use super::u_type::UType;
    #[allow(unused_imports)]
    use super::*;
    use crate::include_generated;
    #[allow(unused_imports)]
    use crate::instr::raw::RawInstr;
    #[allow(unused_imports)]
    use crate::instr::*;
    #[allow(unused_imports)]
    use std::rc::Rc;

    macro_rules! invalid_instr {
        ($instr:expr) => {
            return Err($instr)
        };
    }

    include_generated!("decode_fn.rs");
    include_generated!("decode_impls.rs");
}
