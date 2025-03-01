use crate::components::cpu::reg::RegData;
use crate::instr::Immediate;

pub(super) struct Agu(());

impl Agu {
    pub fn new() -> Self {
        Self(())
    }

    pub fn addr(&self, base: RegData, offset: Immediate) -> RegData {
        RegData::signed(base.i().wrapping_add(offset))
    }
}
