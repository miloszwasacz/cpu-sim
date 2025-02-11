use super::int_reg_imm_instr;
use crate::instr::display_width;

use std::fmt;

int_reg_imm_instr!(Addi, add);

impl Addi {
    const NOP_DISPLAY_NAME: &'static str = "nop";
    const LI_DISPLAY_NAME: &'static str = "li";
    const MV_DISPLAY_NAME: &'static str = "mv";
}

impl fmt::Display for Addi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);

        let is_li = self.src().is_zero();
        let is_mv = self.imm() == 0;

        if self.dest().is_zero() && is_li && is_mv {
            return write!(f, "{:<width$}", Self::NOP_DISPLAY_NAME);
        }

        let name = if is_li {
            Self::LI_DISPLAY_NAME
        } else if is_mv {
            Self::MV_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        self.dest().fmt(f)?;
        if !is_li {
            write!(f, ", ")?;
            self.src().fmt(f)?;
            if is_mv {
                return Ok(());
            }
        }
        write!(f, ", ")?;
        self.imm().fmt(f)
    }
}
