use super::EncodingFormat;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::decode::shared::{decode_rs1, decode_rs2};
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Immediate, Writeback};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct STypeFormat {
    rs1: ArchRegName,
    rs2: ArchRegName,
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

    fn rs1(&self) -> ArchRegName {
        self.rs1
    }

    fn rs2(&self) -> ArchRegName {
        self.rs2
    }

    fn imm(&self) -> Immediate {
        self.imm
    }
}

impl Writeback for STypeFormat {
    fn reg_write(&self) -> bool {
        false
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
