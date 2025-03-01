use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::raw::{Bits, RawInstr, RawInstrBits};

pub(crate) const REG_LEN: u64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Opcode(Bits<{ Opcode::LEN }>);
impl Opcode {
    const LEN: u64 = 7;
    const MSB: u64 = 6;
}
impl From<Opcode> for RawInstrBits {
    fn from(value: Opcode) -> Self {
        value.0.into()
    }
}

pub(crate) const RD_LEN: u64 = REG_LEN;
pub(crate) const RD_MSB: u64 = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Funct3(Bits<{ Funct3::LEN }>);
impl Funct3 {
    const LEN: u64 = 3;
    const MSB: u64 = 14;
}
impl From<Funct3> for RawInstrBits {
    fn from(value: Funct3) -> Self {
        value.0.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Funct7(Bits<{ Funct7::LEN }>);
impl Funct7 {
    const LEN: u64 = 7;
    const MSB: u64 = 31;
}
impl From<Funct7> for RawInstrBits {
    fn from(value: Funct7) -> Self {
        value.0.into()
    }
}

pub(crate) const RS1_LEN: u64 = REG_LEN;
pub(crate) const RS1_MSB: u64 = 19;

pub(crate) const RS2_LEN: u64 = REG_LEN;
pub(crate) const RS2_MSB: u64 = 24;

pub(crate) fn decode_opcode(instr: RawInstr) -> Opcode {
    Opcode(instr.extract_bits::<{ Opcode::LEN }, { Opcode::MSB }>())
}

pub(crate) fn decode_rd(instr: RawInstr) -> ArchRegName {
    instr.extract_bits::<RD_LEN, RD_MSB>().into()
}

pub(crate) fn decode_funct3(instr: RawInstr) -> Funct3 {
    Funct3(instr.extract_bits::<{ Funct3::LEN }, { Funct3::MSB }>())
}

pub(crate) fn decode_rs1(instr: RawInstr) -> ArchRegName {
    instr.extract_bits::<RS1_LEN, RS1_MSB>().into()
}

pub(crate) fn decode_rs2(instr: RawInstr) -> ArchRegName {
    instr.extract_bits::<RS2_LEN, RS2_MSB>().into()
}

pub(crate) fn decode_funct7(instr: RawInstr) -> Funct7 {
    Funct7(instr.extract_bits::<{ Funct7::LEN }, { Funct7::MSB }>())
}
