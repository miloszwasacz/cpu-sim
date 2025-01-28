use crate::instr::Instr;

//#region NOP

pub struct Nop;

impl Nop {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl Instr for Nop {}

//#endregion
