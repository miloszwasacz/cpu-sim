pub use self::add_sub_shift::*;
pub use self::dp_1src::*;
pub use self::dp_2src::*;
pub use self::dp_3src::*;
pub use self::logic_shift::*;
use crate::instr::DisplayOperands;
use crate::reg::RegisterId;

use std::fmt;

mod add_sub_shift;
mod dp_1src;
mod dp_2src;
mod dp_3src;
mod logic_shift;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShiftedRegInstr {
    d: RegisterId,
    n: RegisterId,
    m: RegisterId,
    shift: Shift,
    amount: u32,
}

impl ShiftedRegInstr {
    fn new(d: u32, n: u32, m: u32, sf: u32, shift: u32, imm6: u32) -> Self {
        assert!(
            sf != 0 || imm6 < 32,
            "imm6 has to be in the range 0 to 31 for the 32-bit variant"
        );

        let size = sf.into();
        let d = RegisterId::decode(d, size, false);
        let n = RegisterId::decode(n, size, false);
        let m = RegisterId::decode(m, size, false);
        let shift = shift.into();
        let amount = imm6;

        Self {
            d,
            n,
            m,
            shift,
            amount,
        }
    }
}

impl DisplayOperands for ShiftedRegInstr {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {} #{}",
            self.d, self.n, self.m, self.shift, self.amount
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shift {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl From<u32> for Shift {
    fn from(value: u32) -> Self {
        match value {
            0b00 => Shift::Lsl,
            0b01 => Shift::Lsr,
            0b10 => Shift::Asr,
            0b11 => Shift::Ror,
            val => panic!("{val} is not a valid shift code"),
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_ascii_uppercase())
    }
}
