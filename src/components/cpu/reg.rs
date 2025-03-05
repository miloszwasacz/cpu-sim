pub use self::data::RegData;
pub use self::name::{error, RegName};
pub use self::reg_file::RegFile;
use crate::instr::raw::REG_LEN;

mod data;
mod name;
pub(in crate::components::cpu) mod pipeline;
mod reg_file;

const ARCH_REG_COUNT: usize = 1 << REG_LEN;

#[derive(Debug, Clone, Copy, Default)]
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
}
