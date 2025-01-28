use crate::instr::Instr;
use crate::pipeline::decode::sign_extend;

//#region B

pub struct B {
    offset: i64,
}

impl B {
    pub fn new(imm26: u32) -> Self {
        const ALIGNMENT_SHIFT: u32 = 2;

        let offset = sign_extend(imm26 << ALIGNMENT_SHIFT, 26 + ALIGNMENT_SHIFT);
        Self { offset }
    }
}

impl Instr for B {}

//#endregion
