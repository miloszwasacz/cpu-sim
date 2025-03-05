use super::{delegate_decode, EncodingFormat, STypeFormat};
use crate::components::cpu::reg::RegName;
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Immediate, Writeback};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BTypeFormat(STypeFormat);

impl BTypeFormat {
    const IMM_LEN: u64 = STypeFormat::IMM_LEN;

    fn decode_imm(instr: RawInstr) -> Immediate {
        let imm_12 = instr.extract_bits::<1, 31>().resize::<{ Self::IMM_LEN }>() << 12u32;
        let imm_10_5 = instr.extract_bits::<6, 30>().resize::<{ Self::IMM_LEN }>() << 5u32;
        let imm_4_1 = instr.extract_bits::<4, 11>().resize::<{ Self::IMM_LEN }>() << 1u32;
        let imm_11 = instr.extract_bits::<1, 7>().resize::<{ Self::IMM_LEN }>() << 11u32;
        (imm_12 | imm_11 | imm_10_5 | imm_4_1).into()
    }
}

impl Decode for BTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        Self(STypeFormat::new(raw, Self::decode_imm))
    }

    delegate_decode!();
}

impl Writeback for BTypeFormat {
    fn reg_write(&self) -> bool {
        self.0.reg_write()
    }
}

impl fmt::Display for BTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl EncodingFormat for BTypeFormat {}
