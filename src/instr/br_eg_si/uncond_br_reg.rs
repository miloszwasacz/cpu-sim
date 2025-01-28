use crate::instr::Instr;
use crate::reg::{RegisterId, RegisterSize};

struct UncondBrRegInstr {
    n: RegisterId,
}

impl UncondBrRegInstr {
    fn new(n: u32) -> Self {
        let n = RegisterId::decode(n, RegisterSize::X, false);
        Self { n }
    }
}

//#region BR

pub struct Br(UncondBrRegInstr);

impl Br {
    pub fn new(n: u32) -> Self {
        Self(UncondBrRegInstr::new(n))
    }
}

impl Instr for Br {}

//#endregion

//#region RET

pub struct Ret(UncondBrRegInstr);

impl Ret {
    pub fn new(n: u32) -> Self {
        Self(UncondBrRegInstr::new(n))
    }
}

impl Instr for Ret {}

//#endregion
