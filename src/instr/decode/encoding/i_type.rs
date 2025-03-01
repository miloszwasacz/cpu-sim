use super::EncodingFormat;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::decode::shared::{decode_rd, decode_rs1};
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Immediate, Writeback};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ITypeFormat {
    rd: ArchRegName,
    rs1: ArchRegName,
    imm: Immediate,
}

impl ITypeFormat {
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

    fn rs1(&self) -> ArchRegName {
        self.rs1
    }

    fn rd(&self) -> ArchRegName {
        self.rd
    }

    fn imm(&self) -> Immediate {
        self.imm
    }
}

impl Writeback for ITypeFormat {
    fn reg_write(&self) -> bool {
        true
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
