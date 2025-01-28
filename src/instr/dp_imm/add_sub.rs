use super::DpImmInstr;
use crate::instr::{Instr, impl_display};

macro_rules! add_sub_instr {
    ($name:ident, $d_allow_sp:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        impl_display!($name, |self| &self.0);
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
