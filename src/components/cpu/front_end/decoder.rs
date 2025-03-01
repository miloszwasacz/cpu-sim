use crate::instr::raw::{RawInstr, RawInstrBits};
use crate::instr::Instr;

pub struct Decoder(());

impl Decoder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn decode(&mut self, instr: RawInstrBits) -> DecodeResult {
        if instr == 0 {
            return DecodeResult::NoInstr;
        }

        match RawInstr::new(instr).decode() {
            Ok(instr) => DecodeResult::Ok(instr),
            Err(instr) => DecodeResult::Err(instr),
        }
    }
}

#[derive(Debug, Default)]
pub enum DecodeResult {
    #[default]
    NoInstr,
    Ok(Box<dyn Instr>),
    Err(RawInstr),
}
