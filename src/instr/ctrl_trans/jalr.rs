use crate::instr::decode::ITypeFormat;
use crate::instr::{display_width, Immediate, Instr};
use crate::reg::RegisterName;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jalr(pub(in crate::instr) ITypeFormat);

impl Jalr {
    const JR_DISPLAY_NAME: &'static str = "jr";

    pub fn dest(&self) -> RegisterName {
        self.0.rd
    }

    pub fn base(&self) -> RegisterName {
        self.0.rs1
    }

    pub fn offset(&self) -> Immediate {
        self.0.imm
    }
}

impl Instr for Jalr {}

impl fmt::Display for Jalr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        match (self.dest(), self.offset(), f.alternate()) {
            (RegisterName::Zero, 0, true) => {
                write!(f, "{:<#width$} {:#}", Self::JR_DISPLAY_NAME, self.base())
            }
            (RegisterName::Zero, 0, false) => {
                write!(f, "{:<width$} {}", Self::JR_DISPLAY_NAME, self.base())
            }
            (_, _, true) => {
                write!(
                    f,
                    "{:<#width$} {:#}, {:#}, {}",
                    Self::DISPLAY_NAME,
                    self.dest(),
                    self.base(),
                    self.offset()
                )
            }
            (_, _, false) => {
                write!(
                    f,
                    "{:<width$} {}, {}, {}",
                    Self::DISPLAY_NAME,
                    self.dest(),
                    self.base(),
                    self.offset()
                )
            }
        }
    }
}
