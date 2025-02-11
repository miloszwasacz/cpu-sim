use super::shared::{decode_rd, decode_rs1, Funct3};
use super::EncodingFormat;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::{Bits, RawInstr};
use crate::instr::stall::{write_reg, StallControl};
use crate::instr::Immediate;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShiftType(Bits<{ ShiftType::LEN }>);
impl ShiftType {
    const LEN: u64 = 7;
    const MSB: u64 = 31;

    pub const fn new(value: u64) -> Self {
        Self(Bits::new(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ITypeFormat {
    pub rd: ArchRegName,
    pub rs1: ArchRegName,
    pub imm: Immediate,
}

impl ITypeFormat {
    fn generic_decode(instr: RawInstr, decode_imm: impl FnOnce(RawInstr) -> Immediate) -> Self {
        let rd = decode_rd(instr);
        let rs1 = decode_rs1(instr);
        let imm = decode_imm(instr);

        Self { rd, rs1, imm }
    }

    pub fn decode_shift_instr(instr: RawInstr) -> Self {
        Self::generic_decode(instr, decode_shift_amount)
    }
}

impl EncodingFormat for ITypeFormat {
    fn decode(instr: RawInstr) -> Self {
        Self::generic_decode(instr, decode_imm)
    }
}

impl StallControl for ITypeFormat {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        if self.rs1.is_zero() {
            HashSet::new()
        } else {
            HashSet::from([self.rs1])
        }
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        write_reg(self.rd)
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

pub(crate) trait IType: From<ITypeFormat> {
    const FUNCT3: Funct3;
}

fn decode_imm(instr: RawInstr) -> Immediate {
    instr.extract_bits::<12, 31>().into()
}

fn decode_shift_amount(instr: RawInstr) -> Immediate {
    instr.extract_bits::<5, 24>().into()
}

pub(crate) fn decode_shift_type(instr: RawInstr) -> ShiftType {
    ShiftType(instr.extract_bits::<{ ShiftType::LEN }, { ShiftType::MSB }>())
}
