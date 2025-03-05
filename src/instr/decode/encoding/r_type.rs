use super::EncodingFormat;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_rd, decode_rs1, decode_rs2};
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Writeback};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RTypeFormat {
    rd: RegName,
    rs1: RegName,
    rs2: RegName,
}

impl Decode for RTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        let rd = decode_rd(raw);
        let rs1 = decode_rs1(raw);
        let rs2 = decode_rs2(raw);

        Self { rd, rs1, rs2 }
    }

    fn rs1(&self) -> RegName {
        self.rs1
    }

    fn rs2(&self) -> RegName {
        self.rs2
    }

    fn rd(&self) -> RegName {
        self.rd
    }
}

impl Writeback for RTypeFormat {
    fn reg_write(&self) -> bool {
        true
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
