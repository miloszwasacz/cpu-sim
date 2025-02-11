use super::shared::{decode_rd, decode_rs1, decode_rs2, Funct3};
use super::EncodingFormat;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::{Bits, RawInstr};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Funct7(Bits<{ Funct7::LEN }>);
impl Funct7 {
    const LEN: u64 = 7;
    const MSB: u64 = 31;

    pub const fn new(value: u64) -> Self {
        Self(Bits::new(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RTypeFormat {
    pub rd: ArchRegName,
    pub rs1: ArchRegName,
    pub rs2: ArchRegName,
}

impl EncodingFormat for RTypeFormat {
    fn decode(instr: RawInstr) -> Self {
        let rd = decode_rd(instr);
        let rs1 = decode_rs1(instr);
        let rs2 = decode_rs2(instr);

        Self { rd, rs1, rs2 }
    }
}

impl fmt::Display for RTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd.fmt(f)?;
        write!(f, ", ")?;
        self.rs1.fmt(f)?;
        write!(f, ", ")?;
        self.rs2.fmt(f)
    }
}

pub(crate) trait RType: From<RTypeFormat> {
    const FUNCT3: Funct3;
    const FUNCT7: Funct7;
}

pub(crate) fn decode_funct7(instr: RawInstr) -> Funct7 {
    Funct7(instr.extract_bits::<{ Funct7::LEN }, { Funct7::MSB }>())
}
