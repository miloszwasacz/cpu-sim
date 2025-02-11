use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::Immediate;

pub struct Agu(());

impl Agu {
    pub(super) fn new() -> Self {
        Self(())
    }

    pub fn addr(&self, base: RegData, offset: Immediate) -> Address {
        base.wrapping_add(offset) as Address
    }
}
