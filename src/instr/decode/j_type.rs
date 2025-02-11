use super::{EncodingFormat, UTypeFormat};
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::RawInstr;
use crate::instr::stall::StallControl;
use crate::instr::Immediate;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JTypeFormat(pub UTypeFormat);

impl JTypeFormat {
    const IMM_LEN: u64 = 21;
}

impl EncodingFormat for JTypeFormat {
    fn decode(instr: RawInstr) -> Self {
        Self(UTypeFormat::generic_decode(instr, decode_imm))
    }
}

impl StallControl for JTypeFormat {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        self.0.read_regs()
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        self.0.write_reg()
    }
}

impl fmt::Display for JTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub(crate) trait JType: From<JTypeFormat> {}

fn decode_imm(instr: RawInstr) -> Immediate {
    const IMM_LEN: u64 = JTypeFormat::IMM_LEN;
    let imm_20 = instr.extract_bits::<1, 31>().resize::<IMM_LEN>() << 20u32;
    let imm_10_1 = instr.extract_bits::<10, 30>().resize::<IMM_LEN>() << 1u32;
    let imm_11 = instr.extract_bits::<1, 31>().resize::<IMM_LEN>() << 11u32;
    let imm_19_12 = instr.extract_bits::<8, 19>().resize::<IMM_LEN>() << 12u32;
    (imm_20 | imm_19_12 | imm_11 | imm_10_1).into()
}
