use crate::components::cpu::error::MemAccessError;
use crate::components::cpu::reg::RegData;
use crate::components::memory::{Address, Memory};

pub trait MemoryAccess {
    fn load(&self, _mem: &Memory, _address: Address) -> Result<RegData, MemAccessError> {
        Err(MemAccessError::InvalidLoad)
    }

    fn store(
        &self,
        _mem: &mut Memory,
        _address: Address,
        _data: RegData,
    ) -> Result<(), MemAccessError> {
        Err(MemAccessError::InvalidStore)
    }
}
