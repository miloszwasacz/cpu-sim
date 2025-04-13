use super::EncodingFormat;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::decode_rd;
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;
use crate::instr::Immediate;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UTypeFormat {
    rd: RegName,
    imm: Immediate,
}

impl UTypeFormat {
    const IMM_LEN: u64 = 32;

    pub(super) fn new(instr: RawInstr, decode_imm: impl FnOnce(RawInstr) -> Immediate) -> Self {
        let rd = decode_rd(instr);
        let imm = decode_imm(instr);

        Self { rd, imm }
    }

    #[inline(always)]
    pub fn rd(&self) -> RegName {
        self.rd
    }

    #[inline(always)]
    pub fn imm(&self) -> Immediate {
        self.imm
    }

    fn decode_imm(instr: RawInstr) -> Immediate {
        let imm_31_12 = instr.extract_bits::<20, 31>().resize::<{ Self::IMM_LEN }>() << 12u32;
        imm_31_12.into()
    }
}

impl Decode for UTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        Self::new(raw, Self::decode_imm)
    }
}

impl fmt::Display for UTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd.fmt(f)?;
        write!(f, ", {}", self.imm)
    }
}

impl EncodingFormat for UTypeFormat {}
