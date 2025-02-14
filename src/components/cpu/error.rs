use crate::components::memory::Address;
use crate::instr::raw::{RawInstr, RawInstrBits};

use std::error::Error;
use std::fmt;

const ADDR_DISPLAY_WIDTH: usize = RawInstrBits::BITS as usize / 16;

//#region Fetch

#[derive(Debug, Clone, Copy)]
pub enum FetchError {
    InstructionAddressMisaligned(Address),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstructionAddressMisaligned(addr) => {
                write!(
                    f,
                    "instruction-address-misaligned exception: {addr:#0width$x}",
                    width = ADDR_DISPLAY_WIDTH
                )
            }
        }
    }
}

impl Error for FetchError {}

//#endregion

//#region Decode

#[derive(Debug, Clone, Copy)]
pub enum DecodeError {
    InvalidInstruction(RawInstr),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidInstruction(instr) => {
                write!(f, "{} is an invalid or unsupported instruction", instr)
            }
        }
    }
}

impl Error for DecodeError {}

//#endregion

//#region Execute

#[derive(Debug, Clone, Copy)]
pub enum ExecuteError {}

impl fmt::Display for ExecuteError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ExecuteError {}

//#endregion

//#region Memory Access

#[derive(Debug, Clone, Copy)]
pub enum MemAccessError {
    InvalidLoad,
    InvalidStore,
}

impl fmt::Display for MemAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid memory access: ")?;
        match self {
            Self::InvalidLoad => write!(f, "instruction is not a load"),
            Self::InvalidStore => write!(f, "instruction is not a store"),
        }
    }
}

impl Error for MemAccessError {}

//#endregion

//#region Writeback

#[derive(Debug, Clone, Copy)]
pub enum WritebackError {}

impl fmt::Display for WritebackError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for WritebackError {}

//#endregion
