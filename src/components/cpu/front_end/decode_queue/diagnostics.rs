use super::{DecodeQueue, DecodeQueueEntry};
use crate::components::cpu::error::Exception;
use crate::components::diagnostics::Diagnostics;
use crate::instr::full::FullInstruction;

pub struct DecodeQueueSnapshot {
    pub entries: Box<[Result<FullInstruction, Exception>]>,
    pub capacity: usize,
}

impl Diagnostics for DecodeQueue {
    type Output = DecodeQueueSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self
            .queue
            .iter()
            .map(|entry| match entry {
                DecodeQueueEntry::Ok(decoded) => Ok(decoded.full),
                DecodeQueueEntry::Exception(ex, _) => Err(*ex),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self::Output {
            entries,
            capacity: self.queue.capacity(),
        }
    }
}
