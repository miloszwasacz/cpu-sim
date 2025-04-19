use cpu_sim::components::diagnostics::cpu::{LoadQueueEntry, LoadQueueSnapshot};

pub struct LoadQueueModel(LoadQueueSnapshot);

impl LoadQueueModel {
    pub(super) fn new(load_queue: LoadQueueSnapshot) -> Self {
        Self(load_queue)
    }

    pub fn entries(&self) -> &[LoadQueueEntry] {
        &self.0.0
    }
}
