use crate::components::cpu::csr::CsrAddr;
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
    CsrError(CsrError),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchError(err) => fmt::Display::fmt(err, f),
            Self::DecodeError(err) => fmt::Display::fmt(err, f),
            Self::CsrError(err) => fmt::Display::fmt(err, f),
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
    //TODO Remove when FENCE gets implemented
    Unimplemented(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidInstruction(instr) => {
                write!(f, "{} is an invalid or unsupported instruction", instr)
            }
            DecodeError::Unimplemented(instr) => write!(f, "`{}` is not implemented yet", instr),
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

//#region CsrError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsrError {
    InvalidCsr(CsrAddr),
    WriteError(CsrAddr),
}

impl fmt::Display for CsrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCsr(addr) => {
                write!(f, "CSR {} is not implemented", addr)
            }
            Self::WriteError(addr) => {
                write!(f, "could not write to the {} CSR", addr)
            }
        }
    }
}

impl Error for CsrError {}

impl From<CsrError> for Exception {
    fn from(value: CsrError) -> Self {
        Self::CsrError(value)
    }
}

//#endregion
