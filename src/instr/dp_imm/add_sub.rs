use super::DpImmInstr;
use crate::instr::Instr;

macro_rules! add_sub_instr {
    ($name:ident, $d_allow_sp:expr) => {
        pub struct $name(DpImmInstr<u32>);

        impl $name {
            pub fn new(d: u32, n: u32, sf: u32, sh: u32, imm12: u32) -> Self {
                Self(DpImmInstr::new(
                    (d, $d_allow_sp),
                    (n, true),
                    sf,
                    sh,
                    (imm12, 12),
                ))
            }
        }

        impl Instr for $name {}
    };
}

//#region ADD (immediate)

add_sub_instr!(Add, true);

//#endregion

//#region ADDS (immediate)

add_sub_instr!(Adds, false);

//#endregion

//#region SUB (immediate)

add_sub_instr!(Sub, true);

//#endregion

//#region SUBS (immediate)

add_sub_instr!(Subs, false);

//#endregion
