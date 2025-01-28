use crate::instr::{impl_display, DisplayOperands, Instr};
use crate::pipeline::decode::sign_extend;

use std::fmt;

//#region B

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct B {
    offset: i64,
}

impl B {
    pub fn new(imm26: u32) -> Self {
        const ALIGNMENT_SHIFT: u32 = 2;

        let offset = sign_extend(imm26 << ALIGNMENT_SHIFT, 26 + ALIGNMENT_SHIFT);
        Self { offset }
    }
}

impl Instr for B {}

impl DisplayOperands for B {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.offset)
    }
}

impl_display!(B, |self| self);

//#endregion
