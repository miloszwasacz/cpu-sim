use crate::components::memory::Address;
use crate::instr::raw::{RawInstr, RawInstrBits};
use crate::instr::EnvTrap;

use std::error::Error;
use std::fmt;

const ADDR_DISPLAY_WIDTH: usize = RawInstrBits::BITS as usize / 16;

//#region Exception

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    FetchError(FetchError),
    DecodeError(DecodeError),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchError(err) => fmt::Display::fmt(err, f),
            Self::DecodeError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Error for Exception {}

impl From<Exception> for EnvTrap {
    fn from(value: Exception) -> Self {
        Self::Exception(value)
    }
}

//#endregion

//#region Fetch

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchError {
    MisalignedInstr(Address),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MisalignedInstr(addr) => {
                write!(
                    f,
                    "misaligned instruction address: {addr:#0width$x}",
                    width = ADDR_DISPLAY_WIDTH
                )
            }
        }
    }
}

impl Error for FetchError {}

impl From<FetchError> for Exception {
    fn from(value: FetchError) -> Self {
        Self::FetchError(value)
    }
}

//#endregion

//#region Decode

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<DecodeError> for Exception {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

//#endregion
