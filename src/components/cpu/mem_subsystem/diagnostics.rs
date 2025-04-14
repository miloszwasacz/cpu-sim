use super::LoadQueue;
use crate::components::cpu::exec_engine::RobIndex;
use crate::components::diagnostics::Diagnostics;
use crate::components::memory::Address;

pub struct LoadQueueSnapshot(pub Box<[LoadQueueEntry]>);

pub struct LoadQueueEntry {
    pub addr: Address,
    pub byte_count: usize,
    pub dest: RobIndex,
}

impl Diagnostics for LoadQueue {
    type Output = LoadQueueSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self
            .queue
            .iter()
            .copied()
            .map(LoadQueueEntry::from)
            .collect();

        LoadQueueSnapshot(entries)
    }
}
