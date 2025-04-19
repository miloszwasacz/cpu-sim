use super::LoadQueue;
use crate::components::cpu::exec_engine::{ReorderBuffer, RobIndex};
use crate::components::memory::Address;

pub struct LoadQueueSnapshot(pub Box<[LoadQueueEntry]>);

pub struct LoadQueueEntry {
    pub ready: bool,
    pub addr: Address,
    pub byte_count: usize,
    pub dest: RobIndex,
}

impl LoadQueue {
    pub fn diagnostics(&self, rob: &ReorderBuffer) -> LoadQueueSnapshot {
        let entries = self
            .queue
            .iter()
            .map(|entry| {
                let ready = LoadQueue::is_entry_ready(entry, rob);
                (*entry, ready)
            })
            .map(LoadQueueEntry::from)
            .collect();

        LoadQueueSnapshot(entries)
    }
}
