use super::reg::pipeline::ErrorControl;
use crate::components::memory::Address;
use crate::instr::raw::{RawInstr, RawInstrBits};
use crate::instr::EnvTrap;

use std::error::Error;
use std::fmt;

const ADDR_DISPLAY_WIDTH: usize = RawInstrBits::BITS as usize / 16;

//#region Exception

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    MisalignedInstr(Address),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MisalignedInstr(addr) => {
                write!(
                    f,
                    "instruction-address-misaligned exception: {addr:#0width$x}",
                    width = ADDR_DISPLAY_WIDTH
                )
            }
        }
    }
}

impl Error for Exception {}

impl From<Exception> for EnvTrap {
    fn from(value: Exception) -> Self {
        Self::Exception(value)
    }
}

impl TryFrom<ErrorControl> for Option<Exception> {
    type Error = Box<dyn Error>;

    fn try_from(value: ErrorControl) -> Result<Self, Self::Error> {
        if let Some(err) = value.fetch_error {
            return Ok(Some(match err {
                FetchError::MisalignedInstr(addr) => Exception::MisalignedInstr(addr),
            }));
        }
        if let Some(err) = value.decode_error {
            return match err {
                DecodeError::InvalidInstruction(_) => Err(Box::new(err)),
            };
        }
        if let Some(err) = value.issue_error {
            match err {}
        }
        if let Some(err) = value.execute_error {
            match err {}
        }
        if let Some(err) = value.mem_access_error {
            match err {}
        }
        if let Some(err) = value.writeback_error {
            match err {}
        }

        Ok(None)
    }
}

//#endregion

//#region Fetch

#[derive(Debug, Clone, Copy)]
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

//#region Issue

#[derive(Debug, Clone, Copy)]
pub enum IssueError {}

impl fmt::Display for IssueError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for IssueError {}

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
pub enum MemAccessError {}

impl fmt::Display for MemAccessError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
