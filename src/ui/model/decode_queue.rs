use cpu_sim::components::diagnostics::cpu::{DecodeQueueSnapshot, Exception, Instruction};

pub type Decoded = Result<Instruction, Exception>;

pub struct DecodeQueueModel(DecodeQueueSnapshot);

impl DecodeQueueModel {
    pub(super) fn new(decode_queue: DecodeQueueSnapshot) -> Self {
        Self(decode_queue)
    }

    pub fn entries(&self) -> &[Decoded] {
        &self.0.entries
    }

    pub fn size(&self) -> usize {
        self.0.capacity
    }
}
