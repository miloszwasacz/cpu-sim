use super::EncodingFormat;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_rd, decode_rs1, decode_rs2};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RTypeFormat {
    rd: RegName,
    rs1: RegName,
    rs2: RegName,
}

impl RTypeFormat {
    #[inline(always)]
    pub fn rd(&self) -> RegName {
        self.rd
    }

    #[inline(always)]
    pub fn rs1(&self) -> RegName {
        self.rs1
    }

    #[inline(always)]
    pub fn rs2(&self) -> RegName {
        self.rs2
    }
}

impl Decode for RTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        let rd = decode_rd(raw);
        let rs1 = decode_rs1(raw);
        let rs2 = decode_rs2(raw);

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

impl EncodingFormat for RTypeFormat {}
