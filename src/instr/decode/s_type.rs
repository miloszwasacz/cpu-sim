use super::shared::{decode_rs1, decode_rs2, Funct3};
use super::EncodingFormat;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::RawInstr;
use crate::instr::stall::{read_regs_from_iter, StallControl};
use crate::instr::Immediate;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct STypeFormat {
    pub rs1: ArchRegName,
    pub rs2: ArchRegName,
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

impl StallControl for STypeFormat {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        read_regs_from_iter([self.rs1, self.rs2])
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        None
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

pub(crate) trait SType {
    const FUNCT3: Funct3;
}

fn decode_imm(instr: RawInstr) -> Immediate {
    const IMM_LEN: u64 = STypeFormat::IMM_LEN;
    let imm_11_5 = instr.extract_bits::<7, 31>().resize::<IMM_LEN>() << 5u32;
    let imm_4_0 = instr.extract_bits::<5, 11>().resize::<IMM_LEN>();
    (imm_11_5 | imm_4_0).into()
}
