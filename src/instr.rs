use std::any::Any;

pub mod br_eg_si;
pub mod dp_imm;
pub mod dp_reg;
pub mod ldr_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawInstr(u32);

impl RawInstr {
    pub fn new(bytes: &[u8]) -> RawInstr {
        let bytes = bytes
            .try_into()
            .expect("instructions should be 4 bytes long");
        let instr = u32::from_le_bytes(bytes);
        Self(instr)
    }

    pub fn encoded(&self) -> u32 {
        self.0
    }
}

pub trait Instr: Any {}
