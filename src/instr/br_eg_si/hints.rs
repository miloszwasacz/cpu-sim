use crate::instr::{impl_display, DisplayOperands, Instr};

use std::fmt;

//#region NOP

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nop(());

impl Nop {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(())
    }
}

impl Instr for Nop {}

impl DisplayOperands for Nop {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl_display!(Nop);

//#endregion
