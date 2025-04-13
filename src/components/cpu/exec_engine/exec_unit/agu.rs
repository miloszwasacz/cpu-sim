use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::Immediate;

pub(in crate::components::cpu) struct Agu;

impl Agu {
    pub fn addr(&mut self, base: RegData, offset: Immediate) -> Address {
        let addr = base.i().wrapping_add(offset);
        addr as Address
    }
}
