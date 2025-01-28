use crate::instr::{impl_display, DisplayOperands, Instr};
use crate::reg::RegisterId;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dp1SrcInstr {
    d: RegisterId,
    n: RegisterId,
}

impl Dp1SrcInstr {
    fn new(d: u32, n: u32, sf: u32) -> Self {
        let size = sf.into();
        let d = RegisterId::decode(d, size, false);
        let n = RegisterId::decode(n, size, false);

        Self { d, n }
    }
}

impl DisplayOperands for Dp1SrcInstr {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.d, self.n)
    }
}

macro_rules! dp_1src_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(Dp1SrcInstr);

        impl $name {
            pub fn new(d: u32, n: u32, sf: u32) -> Self {
                Self(Dp1SrcInstr::new(d, n, sf))
            }
        }

        impl Instr for $name {}

        impl_display!($name, |self| &self.0);
    };
}

//#region ABS

dp_1src_instr!(Abs);

//#endregion
