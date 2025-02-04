use crate::instr::decode::JTypeFormat;
use crate::instr::{display_width, Immediate, Instr};
use crate::reg::RegisterName;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jal(pub(in crate::instr) JTypeFormat);

impl Jal {
    const J_DISPLAY_NAME: &'static str = "j";

    pub fn dest(&self) -> RegisterName {
        self.0 .0.rd
    }

    pub fn offset(&self) -> Immediate {
        self.0 .0.imm
    }
}

impl Instr for Jal {}

impl fmt::Display for Jal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        match self.dest() {
            RegisterName::Zero if f.alternate() => {
                write!(f, "{:<#width$} {:#}", Self::J_DISPLAY_NAME, self.offset())
            }
            RegisterName::X(_) if f.alternate() => {
                write!(f, "{:<#width$} {:#}", Self::DISPLAY_NAME, self.0)
            }
            RegisterName::Zero => write!(f, "{:<width$} {}", Self::J_DISPLAY_NAME, self.offset()),
            RegisterName::X(_) => write!(f, "{:<width$} {}", Self::DISPLAY_NAME, self.0),
        }
    }
}
