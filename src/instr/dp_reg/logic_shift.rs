use super::ShiftedRegInstr;
use crate::instr::{impl_display, Instr};

macro_rules! logic_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(ShiftedRegInstr);

        impl $name {
            pub fn new(d: u32, n: u32, m: u32, sf: u32, shift: u32, imm6: u32) -> Self {
                Self(ShiftedRegInstr::new(d, n, m, sf, shift, imm6))
            }
        }

        impl Instr for $name {}

        impl_display!($name, |self| &self.0);
    };
}

//#region AND (shifted register)

logic_instr!(And);

//#endregion

//#region ORR (shifted register)

logic_instr!(Orr);

//#endregion

//#region EOR (shifted register)

logic_instr!(Eor);

//#endregion

//#region ANDS (shifted register)

logic_instr!(Ands);

//#endregion
