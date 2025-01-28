use super::{DpImmInstr, ImmediateShift};
use crate::instr::{impl_display, Instr};

const fn make_simm(imm8: u32) -> (i8, ImmediateShift) {
    let imm8 = imm8 as i32;
    (imm8 as i8, 0)
}

const fn make_uimm(imm8: u32) -> (u8, ImmediateShift) {
    (imm8 as u8, 0)
}

macro_rules! min_max_instr {
    ($name:ident, $ty:ty, $sign:path) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(DpImmInstr<$ty>);

        impl $name {
            pub fn new(d: u32, n: u32, sf: u32, imm8: u32) -> Self {
                Self(DpImmInstr::new((d, false), (n, false), sf, 0, $sign(imm8)))
            }
        }

        impl Instr for $name {}

        impl_display!($name, |self| &self.0);
    };
    ($name:ident::<S>) => {
        min_max_instr!($name, i8, make_simm);
    };
    ($name:ident::<U>) => {
        min_max_instr!($name, u8, make_uimm);
    };
}

//#region SMAX (immediate)

min_max_instr!(Smax::<S>);

//#endregion

//#region UMAX (immediate)

min_max_instr!(Umax::<U>);

//#endregion

//#region SMIN (immediate)

min_max_instr!(Smin::<S>);

//#endregion

//#region UMIN (immediate)

min_max_instr!(Umin::<U>);

//#endregion
