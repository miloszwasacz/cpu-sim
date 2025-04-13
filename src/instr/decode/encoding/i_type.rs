use super::EncodingFormat;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_rd, decode_rs1};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;
use crate::instr::Immediate;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ITypeFormat {
    rd: RegName,
    rs1: RegName,
    imm: Immediate,
}

impl ITypeFormat {
    #[inline(always)]
    pub fn rd(&self) -> RegName {
        self.rd
    }

    #[inline(always)]
    pub fn rs1(&self) -> RegName {
        self.rs1
    }

    #[inline(always)]
    pub fn imm(&self) -> Immediate {
        self.imm
    }

    fn decode_imm(instr: RawInstr) -> Immediate {
        instr.extract_bits::<12, 31>().into()
    }
}

impl Decode for ITypeFormat {
    fn decode(raw: RawInstr) -> Self {
        let rd = decode_rd(raw);
        let rs1 = decode_rs1(raw);
        let imm = Self::decode_imm(raw);

        Self { rd, rs1, imm }
    }
}

impl fmt::Display for ITypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd.fmt(f)?;
        write!(f, ", ")?;
        self.rs1.fmt(f)?;
        write!(f, ", {}", self.imm)
    }
}

impl EncodingFormat for ITypeFormat {}
