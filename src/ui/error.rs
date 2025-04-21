use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum SimError {
    EarlyExit,
    Tui(io::Error),
    Cpu(Vec<Box<dyn Error>>),
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimError::EarlyExit => fmt::Display::fmt("early exit", f),
            SimError::Tui(err) => fmt::Display::fmt(err, f),
            SimError::Cpu(errs) => {
                for err in errs {
                    fmt::Display::fmt(err, f)?;
                    fmt::Display::fmt("\n", f)?;
                }
                Ok(())
            }
        }
    }
}

impl Error for SimError {}

impl From<io::Error> for SimError {
    fn from(err: io::Error) -> Self {
        Self::Tui(err)
    }
}

impl From<Vec<Box<dyn Error>>> for SimError {
    fn from(err: Vec<Box<dyn Error>>) -> Self {
        Self::Cpu(err)
    }
}
