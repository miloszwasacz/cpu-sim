use std::any::Any;
use std::fmt;

pub mod br_eg_si;
pub mod dp_imm;
pub mod dp_reg;
pub mod ldr_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawInstr(u32);

impl RawInstr {
    pub fn new(bytes: &[u8]) -> RawInstr {
        let bytes = bytes
            .try_into()
            .expect("instructions should be 4 bytes long");
        let instr = u32::from_le_bytes(bytes);
        Self(instr)
    }

    pub fn encoded(&self) -> u32 {
        self.0
    }
}

pub trait Instr: Any + fmt::Debug + fmt::Display {}

macro_rules! const_to_upper {
    ($tokens:tt) => {
        const_format::map_ascii_case!(const_format::Case::Upper, stringify!($tokens))
    };
}
use const_to_upper;

const INSTR_PRETTY_WIDTH: usize = 10;

macro_rules! impl_display {
    ($instr:ty) => {
        impl std::fmt::Display for $instr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = if f.alternate() { crate::instr::INSTR_PRETTY_WIDTH } else { 0 };
                write!(f, "{:<width$}", crate::instr::const_to_upper!($instr))
            }
        }
    };
    ($instr:ty, |$self:ident| $operand_src:expr) => {
        impl std::fmt::Display for $instr {
            fn fmt(&$self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = if f.alternate() { crate::instr::INSTR_PRETTY_WIDTH } else { 0 };
                write!(f, "{:<width$} ", crate::instr::const_to_upper!($instr))?;
                crate::instr::DisplayOperands::write_operands($operand_src, f)
            }
        }
    };
}
use impl_display;

trait DisplayOperands {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
