use super::DpImmInstr;
use crate::instr::{impl_display, Instr};
use crate::pipeline::decode::{ones, replicate, ror};

const BITS6: u32 = 0b111111;

#[inline]
fn make_imm(n: u32, imms: u32, immr: u32, m: u32) -> u64 {
    let len = ((n << 6) | ((!imms) & BITS6)).ilog2();
    assert!(len >= 1, "undefined behavior");
    assert!(m >= (1 << len));

    let levels = ones(len) as u32 & BITS6;

    assert_ne!(imms & levels, levels, "undefined behavior");

    let s = imms & levels;
    let r = immr & levels;

    let esize = 1 << len;
    let welem = ones(s + 1);

    replicate(ror(welem, esize, r), esize, m / esize)
}

macro_rules! logical_instr {
    ($name:ident, $d_allow_sp:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(DpImmInstr<u64>);

        impl $name {
            pub fn new(rd: u32, rn: u32, sf: u32, n: u32, immr: u32, imms: u32) -> Self {
                let imm = make_imm(n, imms, immr, 32 << sf);
                Self(DpImmInstr::new(
                    (rd, $d_allow_sp),
                    (rn, false),
                    sf,
                    0,
                    (imm, 0),
                ))
            }
        }

        impl Instr for $name {}

        impl_display!($name, |self| &self.0);
    };
    ($name:ident) => {
        logical_instr!($name, true);
    };
}

//#region AND (immediate)

logical_instr!(And);

//#endregion

//#region ORR (immediate)

logical_instr!(Orr);

//#endregion

//#region EOR (immediate)

logical_instr!(Eor);

//#endregion

//#region ANDS (immediate)

logical_instr!(Ands, false);

//#endregion
