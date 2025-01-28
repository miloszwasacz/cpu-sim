pub use self::add_sub::*;
pub use self::logical::*;
pub use self::min_max::*;
pub use self::move_wide::*;
use crate::instr::DisplayOperands;
use crate::reg::RegisterId;

use std::fmt::{self, Formatter};
use std::ops::Shl;

mod add_sub;
mod logical;
mod min_max;
mod move_wide;

type AllowSP = bool;
type ImmediateShift = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DpImmInstr<T> {
    d: RegisterId,
    n: RegisterId,
    imm: T,
}

impl<T: Shl<u32, Output = T>> DpImmInstr<T> {
    pub fn new(
        d: (u32, AllowSP),
        n: (u32, AllowSP),
        sf: u32,
        sh: u32,
        imm: (T, ImmediateShift),
    ) -> Self {
        let size = sf.into();
        let d = RegisterId::decode(d.0, size, d.1);
        let n = RegisterId::decode(n.0, size, n.1);

        let imm = match sh {
            0 => imm.0,
            1 => imm.0 << imm.1,
            sh => panic!("{sh} is not a valid shift value"),
        };

        DpImmInstr { d, n, imm }
    }
}

impl<T: fmt::Display> DisplayOperands for DpImmInstr<T> {
    fn write_operands(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, #{}", self.d, self.n, self.imm)
    }
}
