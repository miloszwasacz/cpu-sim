use super::EncodingFormat;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_rs1, decode_rs2};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;
use crate::instr::Immediate;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct STypeFormat {
    rs1: RegName,
    rs2: RegName,
    imm: Immediate,
}

impl STypeFormat {
    pub(super) const IMM_LEN: u64 = 12;

    pub(super) fn new(instr: RawInstr, decode_imm: impl FnOnce(RawInstr) -> Immediate) -> Self {
        let rs1 = decode_rs1(instr);
        let rs2 = decode_rs2(instr);
        let imm = decode_imm(instr);

        Self { rs1, rs2, imm }
    }

    #[inline(always)]
    pub fn rs1(&self) -> RegName {
        self.rs1
    }

    #[inline(always)]
    pub fn rs2(&self) -> RegName {
        self.rs2
    }

    #[inline(always)]
    pub fn imm(&self) -> Immediate {
        self.imm
    }

    fn decode_imm(instr: RawInstr) -> Immediate {
        let imm_11_5 = instr.extract_bits::<7, 31>().resize::<{ Self::IMM_LEN }>() << 5u32;
        let imm_4_0 = instr.extract_bits::<5, 11>().resize::<{ Self::IMM_LEN }>();
        (imm_11_5 | imm_4_0).into()
    }
}

impl Decode for STypeFormat {
    fn decode(raw: RawInstr) -> Self {
        Self::new(raw, Self::decode_imm)
    }
}

impl fmt::Display for STypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rs1.fmt(f)?;
        write!(f, ", ")?;
        self.rs2.fmt(f)?;
        write!(f, ", {}", self.imm)
    }
}

impl EncodingFormat for STypeFormat {}
