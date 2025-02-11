use crate::components::cpu::error::ExecuteError;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::components::cpu::reg::{RegData, RegFile};
use crate::components::cpu::{Agu, Alu, ProgramCounter};
use crate::components::memory::Address;

pub trait Execute {
    fn execute(
        &self,
        reg_file: &dyn RegFile<Index = ArchRegName>,
        pc: &mut ProgramCounter,
        alu: &mut Alu,
        load_agu: &mut Agu,
        store_agu: &mut Agu,
    ) -> Result<ExecuteResult, ExecuteError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteResult {
    Alu(ArchRegName, RegData),
    Branch,
    LoadAgu(ArchRegName, Address),
    StoreAgu(Address, RegData),
    Ecall,
    Ebreak,
}
