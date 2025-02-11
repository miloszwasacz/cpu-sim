use crate::instr::raw::REG_LEN;

pub mod arf;
// pub mod prf;

const ARCH_REG_COUNT: usize = 1 << REG_LEN;

pub type RegData = i32;
pub type RegDataUnsigned = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Register(RegData);

impl Register {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self) -> RegData {
        self.0
    }

    pub fn set(&mut self, data: RegData) {
        self.0 = data;
    }

    pub fn get_unsigned(&self) -> RegDataUnsigned {
        self.get() as RegDataUnsigned
    }

    pub fn set_unsigned(&mut self, data: RegDataUnsigned) {
        self.0 = data as RegData
    }
}

pub trait RegFile {
    type Index;
    
    fn get(&self, reg: Self::Index) -> &Register;
    fn set(&mut self, reg: Self::Index, data: RegData);
}
