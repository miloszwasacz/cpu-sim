use super::DecodeOption;
use crate::instr::raw::RawInstr;
use crate::instr::Instr;

use std::rc::Rc;

pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&self, instr: RawInstr) -> DecodeOption<Rc<dyn Instr>> {
        instr
            .decode()
            .map(DecodeOption::Some)
            .unwrap_or_else(DecodeOption::InvalidInstr)
    }
}
