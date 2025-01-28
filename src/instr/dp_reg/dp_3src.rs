use crate::instr::{impl_display, DisplayOperands, Instr};
use crate::reg::RegisterId;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dp3SrcInst {
    d: RegisterId,
    n: RegisterId,
    m: RegisterId,
    a: RegisterId,
}

impl Dp3SrcInst {
    fn new(d: u32, n: u32, m: u32, a: u32, sf: u32) -> Self {
        let size = sf.into();
        let d = RegisterId::decode(d, size, false);
        let n = RegisterId::decode(n, size, false);
        let m = RegisterId::decode(m, size, false);
        let a = RegisterId::decode(a, size, false);

        Self { d, n, m, a }
    }
}

impl DisplayOperands for Dp3SrcInst {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.d, self.n, self.m, self.a)
    }
}

macro_rules! dp_3src_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(Dp3SrcInst);

        impl $name {
            pub fn new(d: u32, n: u32, m: u32, a: u32, sf: u32) -> Self {
                Self(Dp3SrcInst::new(d, n, m, a, sf))
            }
        }

        impl Instr for $name {}

        impl_display!($name, |self| &self.0);
    };
}

//#region MADD

dp_3src_instr!(Madd);

//#endregion

//#region MSUB

dp_3src_instr!(Msub);

//#endregion
