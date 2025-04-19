use crate::components::cpu::exec_engine::exec_unit::ExecResult;
use crate::components::cpu::exec_engine::{RobIndex, StoreMeta};
use crate::components::memory::{Address, Memory};
use crate::instr::mem_access::MemRead;

#[derive(Debug, Clone, Copy)]
pub(in crate::components::cpu) struct LoadQueueEntry {
    tag: RobIndex,
    load: MemRead,
    byte_count: usize,
    addr: Address,
}

impl LoadQueueEntry {
    pub fn new(tag: RobIndex, load: MemRead, byte_count: usize, addr: Address) -> Self {
        Self {
            tag,
            load,
            byte_count,
            addr,
        }
    }

    #[inline]
    pub(super) fn tag(&self) -> RobIndex {
        self.tag
    }

    pub(super) fn has_hazard(&self, store: StoreMeta) -> bool {
        let store_start = match store.addr {
            Some(addr) => addr as usize,
            None => return true,
        };
        let store_end = store_start + store.byte_count;

        let load_start = self.addr as usize;
        let load_end = load_start + self.byte_count;

        // Check if addresses of the load and the store overlap
        load_start < store_end && store_start < load_end
    }

    pub(super) fn execute(self, mem: &Memory) -> ExecResult {
        let load = self.load;
        let data = load(mem, self.addr);

        //TODO Exceptions
        ExecResult::load2(self.tag, Ok(data))
    }
}

impl From<(LoadQueueEntry, bool)> for super::diagnostics::LoadQueueEntry {
    fn from((value, ready): (LoadQueueEntry, bool)) -> Self {
        Self {
            ready,
            addr: value.addr,
            byte_count: value.byte_count,
            dest: value.tag,
        }
    }
}
