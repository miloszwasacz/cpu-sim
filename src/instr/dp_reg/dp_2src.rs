use crate::instr::{impl_display, DisplayOperands, Instr};
use crate::reg::RegisterId;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dp2SrcInstr {
    d: RegisterId,
    n: RegisterId,
    m: RegisterId,
}

impl Dp2SrcInstr {
    fn new(d: u32, n: u32, m: u32, sf: u32) -> Self {
        let size = sf.into();
        let d = RegisterId::decode(d, size, false);
        let n = RegisterId::decode(n, size, false);
        let m = RegisterId::decode(m, size, false);

        Self { d, n, m }
    }
}

impl DisplayOperands for Dp2SrcInstr {
    fn write_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.d, self.n, self.m)
    }
}

macro_rules! dp_2src_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(Dp2SrcInstr);

        impl $name {
            pub fn new(d: u32, n: u32, m: u32, sf: u32) -> Self {
                Self(Dp2SrcInstr::new(d, n, m, sf))
            }
        }

        impl Instr for $name {}
        
        impl_display!($name, |self| &self.0);
    };
}

//#region UDIV

dp_2src_instr!(Udiv);

//#endregion

//#region SDIV

dp_2src_instr!(Sdiv);

//#endregion

//#region LSLV

dp_2src_instr!(Lslv);

//#endregion

//#region LSRV

dp_2src_instr!(Lsrv);

//#endregion

//#region ASRV

dp_2src_instr!(Asrv);

//#endregion

//#region RORV

dp_2src_instr!(Rorv);

//#endregion

//#region SMAX

dp_2src_instr!(Smax);

//#endregion

//#region UMAX

dp_2src_instr!(Umax);

//#endregion

//#region SMIN

dp_2src_instr!(Smin);

//#endregion

//#region UMIN

dp_2src_instr!(Umin);

//#endregion
