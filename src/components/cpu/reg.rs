pub use self::data::{DataType, RegData};
pub use self::name::RegName;
pub use self::reg_file::{RegFile, FutureFile};
pub(super) use self::reg_file::FutureFileIssueLock;
use super::flip_flop::{FlipFlop, Sequential};
use crate::instr::raw::REG_LEN;

mod data;
pub mod diagnostics;
mod name;
pub(super) mod pipeline;
mod reg_file;

const ARCH_REG_COUNT: usize = 1 << REG_LEN;

#[derive(Debug, Clone, Copy)]
pub struct Register(FlipFlop<RegData>);

impl Register {
    pub fn get(&self) -> RegData {
        *self.0.read()
    }

    pub fn set(&mut self, data: RegData) {
        self.0.write(data);
    }
}

impl Default for Register {
    fn default() -> Self {
        Self(FlipFlop::new(RegData::default()))
    }
}

impl Sequential for Register {
    fn finish_cycle(&mut self) {
        self.0.finish_cycle();
    }
}
