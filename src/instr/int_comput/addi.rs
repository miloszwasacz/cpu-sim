use super::int_reg_imm_instr;
use crate::instr::display_width;
use crate::reg::RegisterName;

use std::fmt;

int_reg_imm_instr!(Addi);

impl Addi {
    const NOP_DISPLAY_NAME: &'static str = "nop";
    const LI_DISPLAY_NAME: &'static str = "li";
    const MV_DISPLAY_NAME: &'static str = "mv";
}

impl fmt::Display for Addi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        match (self.dest(), self.src(), self.imm(), f.alternate()) {
            (RegisterName::Zero, RegisterName::Zero, 0, true) => {
                write!(f, "{:<#width$}", Self::NOP_DISPLAY_NAME,)
            }
            (RegisterName::Zero, RegisterName::Zero, 0, false) => {
                write!(f, "{:<width$}", Self::NOP_DISPLAY_NAME,)
            }
            (_, RegisterName::Zero, _, true) => {
                write!(
                    f,
                    "{:<#width$} {:#}, {}",
                    Self::LI_DISPLAY_NAME,
                    self.dest(),
                    self.imm()
                )
            }
            (_, RegisterName::Zero, _, false) => {
                write!(
                    f,
                    "{:<width$} {}, {}",
                    Self::LI_DISPLAY_NAME,
                    self.dest(),
                    self.imm()
                )
            }
            (_, _, 0, true) => {
                write!(
                    f,
                    "{:<#width$} {:#}, {:#}",
                    Self::MV_DISPLAY_NAME,
                    self.dest(),
                    self.src()
                )
            }
            (_, _, 0, false) => {
                write!(
                    f,
                    "{:<width$} {}, {}",
                    Self::MV_DISPLAY_NAME,
                    self.dest(),
                    self.src()
                )
            }
            (_, _, _, true) => {
                write!(
                    f,
                    "{:<#width$} {:#}, {:#}, {}",
                    Self::DISPLAY_NAME,
                    self.dest(),
                    self.src(),
                    self.imm()
                )
            }
            (_, _, _, false) => {
                write!(
                    f,
                    "{:<width$} {}, {}, {}",
                    Self::DISPLAY_NAME,
                    self.dest(),
                    self.src(),
                    self.imm()
                )
            }
        }
    }
}
