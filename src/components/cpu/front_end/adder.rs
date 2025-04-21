use super::PcPlus4;
use crate::components::cpu::exec_engine::exec_unit::alu::Alu;
use crate::components::cpu::reg::RegData;
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::raw::RawInstrBits;
use crate::instr::Immediate;

//#region PcAdder

pub struct PcAdder(());

impl PcAdder {
    pub fn new() -> Self {
        Self(())
    }

    pub fn add(&mut self, pc: Pc) -> PcPlus4 {
        pc + size_of::<RawInstrBits>() as PcPlus4
    }
}

//#endregion

//#region JumpAgu

#[derive(Clone)]
pub struct JumpAgu(Alu);

impl JumpAgu {
    pub fn new() -> Self {
        Self(Alu)
    }

    pub fn jump_target(&mut self, pc: Pc, offset: Immediate, apply_mask: bool) -> Address {
        let base = RegData::address(pc);
        self.0.jump_target(base, offset, apply_mask)
    }
}

//#endregion
