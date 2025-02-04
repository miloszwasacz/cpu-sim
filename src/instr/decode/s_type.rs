use super::shared::{decode_rs1, decode_rs2, Funct3};
use super::EncodingFormat;
use crate::instr::raw::RawInstr;
use crate::instr::Immediate;
use crate::reg::RegisterName;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct STypeFormat {
    pub rs1: RegisterName,
    pub rs2: RegisterName,
    pub imm: Immediate,
}

impl STypeFormat {
    pub(crate) const IMM_LEN: u64 = 12;

    pub(crate) fn generic_decode(
        instr: RawInstr,
        decode_imm: impl FnOnce(RawInstr) -> Immediate,
    ) -> Self {
        let rs1 = decode_rs1(instr);
        let rs2 = decode_rs2(instr);
        let imm = decode_imm(instr);

        Self { rs1, rs2, imm }
    }
}

impl EncodingFormat for STypeFormat {
    fn decode(instr: RawInstr) -> Self {
        Self::generic_decode(instr, decode_imm)
    }
}

impl fmt::Display for STypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#}, {:#}, {}", self.rs1, self.rs2, self.imm)
        } else {
            write!(f, "{}, {}, {}", self.rs1, self.rs2, self.imm)
        }
    }
}

pub(crate) trait SType {
    const FUNCT3: Funct3;
}

fn decode_imm(instr: RawInstr) -> Immediate {
    const IMM_LEN: u64 = STypeFormat::IMM_LEN;
    let imm_11_5 = instr.extract_bits::<7, 31>().resize::<IMM_LEN>() << 5u32;
    let imm_4_0 = instr.extract_bits::<5, 11>().resize::<IMM_LEN>();
    (imm_11_5 | imm_4_0).into()
}
