use crate::instr::decode::decode;
use crate::instr::raw::RawInstr;
use crate::instr::Instr;

pub struct Decoder;

impl Decoder {
    pub fn decode(&self, instr: RawInstr) -> Box<dyn Instr> {
        decode(instr)
    }
}
