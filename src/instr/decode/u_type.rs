use super::shared::decode_rd;
use super::EncodingFormat;
use crate::instr::raw::RawInstr;
use crate::instr::Immediate;
use crate::reg::RegisterName;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UTypeFormat {
    pub rd: RegisterName,
    pub imm: Immediate,
}

impl UTypeFormat {
    const IMM_LEN: u64 = 32;

    pub(crate) fn generic_decode(
        instr: RawInstr,
        decode_imm: impl FnOnce(RawInstr) -> Immediate,
    ) -> Self {
        let rd = decode_rd(instr);
        let imm = decode_imm(instr);
        Self { rd, imm }
    }
}

impl EncodingFormat for UTypeFormat {
    fn decode(instr: RawInstr) -> Self {
        Self::generic_decode(instr, decode_imm)
    }
}

impl fmt::Display for UTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#}, {}", self.rd, self.imm)
        } else {
            write!(f, "{}, {}", self.rd, self.imm)
        }
    }
}

pub(crate) trait UType: From<UTypeFormat> {}

fn decode_imm(instr: RawInstr) -> Immediate {
    const IMM_LEN: u64 = UTypeFormat::IMM_LEN;
    let imm_31_12 = instr.extract_bits::<20, 31>().resize::<IMM_LEN>() << 12u32;
    imm_31_12.into()
}
