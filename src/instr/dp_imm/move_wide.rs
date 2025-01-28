use crate::instr::Instr;
use crate::reg::{RegisterId, RegisterSize};

struct Mov {
    d: RegisterId,
    imm: u16,
    shift: u32,
}

impl Mov {
    fn new(d: u32, sf: u32, imm16: u32, hw: u32) -> Self {
        let size = RegisterSize::from(sf);
        let d = RegisterId::decode(d, size, false);
        let imm = imm16 as u16;
        let shift = hw << 4;
        Mov { d, imm, shift }
    }
}

macro_rules! move_instr {
    ($name:ident) => {
        pub struct $name(Mov);
        
        impl $name {
            pub fn new(d: u32, sf: u32, imm16: u32, hw: u32) -> Self {
                Self(Mov::new(d, sf, imm16, hw))
            }
        }
        
        impl Instr for $name {}
    };
}

//#region MOVZ

move_instr!(Movz);

//#endregion
