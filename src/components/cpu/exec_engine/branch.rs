use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::Immediate;

pub(super) struct BranchUnit(());

impl BranchUnit {
    pub fn new() -> Self {
        Self(())
    }

    pub fn target(&self, base: RegData, offset: Immediate, mask_jump_target: bool) -> Address {
        let mut target = base.i().wrapping_add(offset);
        if mask_jump_target {
            target &= !0b1;
        }
        target as Address
    }
}
