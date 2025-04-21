use super::{DecodeQueue, DecodeQueueEntry, Decoded};
use crate::components::cpu::error::Exception;
use crate::components::cpu::exec_engine::OperationType;
use crate::components::cpu::Pc;

pub struct DecodeQueueLock<'a> {
    queue: &'a mut DecodeQueue,
    n: usize,
}

//TODO Add a note mentioning that the destructor HAS TO RUN for the issued instructions to actually get popped
impl<'a> DecodeQueueLock<'a> {
    pub(super) fn new(queue: &'a mut DecodeQueue) -> Self {
        Self { queue, n: 0 }
    }

    pub fn head<'l>(&'l mut self) -> Option<Result<DecodedLock<'a, 'l>, (Exception, Pc)>> {
        let entry = *self.queue.queue.get(self.n)?;
        Some(match entry {
            DecodeQueueEntry::Ok(decoded) => Ok(DecodedLock(self, decoded)),
            DecodeQueueEntry::Exception(ex, pc) => {
                self.n += 1;
                Err((ex, pc))
            }
        })
    }
}

impl Drop for DecodeQueueLock<'_> {
    fn drop(&mut self) {
        debug_assert!(self.queue.popped == 0);
        self.queue.popped = self.n;
    }
}

pub struct DecodedLock<'a, 'l>(&'l mut DecodeQueueLock<'a>, Decoded);

impl DecodedLock<'_, '_> {
    pub fn ty(&self) -> OperationType {
        self.1.instr.ty()
    }

    pub fn issue(self) -> Decoded {
        self.0.n += 1;
        self.1
    }
}
