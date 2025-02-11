use super::shared::Funct3;
use super::{EncodingFormat, STypeFormat};
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::RawInstr;
use crate::instr::stall::StallControl;
use crate::instr::Immediate;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTypeFormat(pub STypeFormat);

impl EncodingFormat for BTypeFormat {
    fn decode(instr: RawInstr) -> Self {
        Self(STypeFormat::generic_decode(instr, decode_imm))
    }
}

impl StallControl for BTypeFormat {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        self.0.read_regs()
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        self.0.write_reg()
    }
}

impl fmt::Display for BTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub(crate) trait BType: From<BTypeFormat> {
    const FUNCT3: Funct3;
}

fn decode_imm(instr: RawInstr) -> Immediate {
    const IMM_LEN: u64 = STypeFormat::IMM_LEN;
    let imm_12 = instr.extract_bits::<1, 31>().resize::<IMM_LEN>() << 12u32;
    let imm_10_5 = instr.extract_bits::<6, 30>().resize::<IMM_LEN>() << 5u32;
    let imm_4_1 = instr.extract_bits::<4, 11>().resize::<IMM_LEN>() << 1u32;
    let imm_11 = instr.extract_bits::<1, 7>().resize::<IMM_LEN>() << 11u32;
    (imm_12 | imm_11 | imm_10_5 | imm_4_1).into()
}
