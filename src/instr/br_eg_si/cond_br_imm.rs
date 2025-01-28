use crate::instr::Instr;
use crate::pipeline::decode::sign_extend;

//#region B.cond

pub struct Bcond {
    cond: Cond,
    offset: i64,
}

impl Bcond {
    pub fn new(cond: u32, imm19: u32) -> Self {
        const ALIGNMENT_SHIFT: u32 = 2;

        let cond = cond.into();
        let offset = sign_extend(imm19 << ALIGNMENT_SHIFT, 19 + ALIGNMENT_SHIFT);
        Self { cond, offset }
    }
}

impl Instr for Bcond {}

pub enum Cond {
    Eq,
    Ne,
    Cs,
    Cc,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
    Nv,
}

impl From<u32> for Cond {
    fn from(value: u32) -> Self {
        match value {
            0b0000 => Cond::Eq,
            0b0001 => Cond::Ne,
            0b0010 => Cond::Cs,
            0b0011 => Cond::Cc,
            0b0100 => Cond::Mi,
            0b0101 => Cond::Pl,
            0b0110 => Cond::Vs,
            0b0111 => Cond::Vc,
            0b1000 => Cond::Hi,
            0b1001 => Cond::Ls,
            0b1010 => Cond::Ge,
            0b1011 => Cond::Lt,
            0b1100 => Cond::Gt,
            0b1101 => Cond::Le,
            0b1110 => Cond::Al,
            0b1111 => Cond::Nv,
            val => panic!("{val} is not a valid condition code"),
        }
    }
}

//#endregion
