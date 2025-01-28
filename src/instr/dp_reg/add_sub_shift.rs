use super::ShiftedRegInstr;
use crate::instr::Instr;

macro_rules! add_sub_instr {
    ($name:ident) => {
        pub struct $name(ShiftedRegInstr);

        impl $name {
            pub fn new(d: u32, n: u32, m: u32, sf: u32, shift: u32, imm6: u32) -> Self {
                assert!(shift != 0b11, "shift code '11' is reserved");
                Self(ShiftedRegInstr::new(d, n, m, sf, shift, imm6))
            }
        }

        impl Instr for $name {}
    };
}

//#region ADD (shifted register)

add_sub_instr!(Add);

//#endregion

//#region ADDS (shifted register)

add_sub_instr!(Adds);

//#endregion

//#region SUB (shifted register)

add_sub_instr!(Sub);

//#endregion

//#region SUBS (shifted register)

add_sub_instr!(Subs);

//#endregion
