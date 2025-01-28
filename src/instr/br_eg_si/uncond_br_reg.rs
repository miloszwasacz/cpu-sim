use crate::instr::{impl_display, DisplayOperands, Instr};
use crate::reg::{RegisterId, RegisterSize};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UncondBrRegInstr {
    n: RegisterId,
}

impl UncondBrRegInstr {
    fn new(n: u32) -> Self {
        let n = RegisterId::decode(n, RegisterSize::X, false);
        Self { n }
    }
}

impl DisplayOperands for UncondBrRegInstr {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.n)
    }
}

//#region BR

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Br(UncondBrRegInstr);

impl Br {
    pub fn new(n: u32) -> Self {
        Self(UncondBrRegInstr::new(n))
    }
}

impl Instr for Br {}

impl_display!(Br, |self| &self.0);

//#endregion

//#region RET

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ret(UncondBrRegInstr);

impl Ret {
    pub fn new(n: u32) -> Self {
        Self(UncondBrRegInstr::new(n))
    }
}

impl Instr for Ret {}

impl_display!(Ret, |self| &self.0);

//#endregion
