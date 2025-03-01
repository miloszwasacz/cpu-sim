use super::{delegate_decode, EncodingFormat, UTypeFormat};
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Immediate, Writeback};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JTypeFormat(UTypeFormat);

impl JTypeFormat {
    const IMM_LEN: u64 = 21;

    fn decode_imm(instr: RawInstr) -> Immediate {
        let imm_20 = instr.extract_bits::<1, 31>().resize::<{ Self::IMM_LEN }>() << 20u32;
        let imm_10_1 = instr.extract_bits::<10, 30>().resize::<{ Self::IMM_LEN }>() << 1u32;
        let imm_11 = instr.extract_bits::<1, 20>().resize::<{ Self::IMM_LEN }>() << 11u32;
        let imm_19_12 = instr.extract_bits::<8, 19>().resize::<{ Self::IMM_LEN }>() << 12u32;
        (imm_20 | imm_19_12 | imm_11 | imm_10_1).into()
    }
}

impl Decode for JTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        Self(UTypeFormat::new(raw, Self::decode_imm))
    }

    delegate_decode!();
}

impl Writeback for JTypeFormat {
    fn reg_write(&self) -> bool {
        self.0.reg_write()
    }
}

impl fmt::Display for JTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl EncodingFormat for JTypeFormat {}
