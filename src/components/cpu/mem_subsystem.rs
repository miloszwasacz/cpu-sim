pub(super) use self::entry::LoadQueueEntry;
use super::exec_engine::exec_unit::ExecResult;
use super::exec_engine::ReorderBuffer;
use super::flip_flop::{Clearable, Sequential};
use crate::components::memory::Memory;

use itertools::Itertools;
use std::collections::VecDeque;
use std::mem;
use std::num::NonZeroUsize;

pub mod diagnostics;
mod entry;

pub struct LoadQueue {
    queue: VecDeque<LoadQueueEntry>,
    scheduled: Vec<LoadQueueEntry>,
    dequeued: bool,
    cleared: bool,
}

impl LoadQueue {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        LoadQueue {
            queue: VecDeque::with_capacity(capacity.get()),
            scheduled: Vec::new(),
            dequeued: false,
            cleared: false,
        }
    }

    pub fn free_spaces(&self) -> usize {
        debug_assert!(!self.cleared);
        self.queue.capacity() - self.queue.len()
    }

    pub fn execute(&mut self, rob: &ReorderBuffer, mem: &Memory) -> Option<ExecResult> {
        debug_assert!(!self.cleared);
        self.queue
            .iter()
            .find(|load| {
                // There can be no RAW hazards caused by preceding stores or traps
                !rob.preceding_stores(load.tag())
                    .map_ok(|store| load.has_hazard(store))
                    .any(|hazard| hazard.unwrap_or(true))
            })
            .map(|instr| {
                self.dequeued = true;
                instr.execute(mem)
            })
    }

    pub fn write(&mut self, results: Vec<LoadQueueEntry>) {
        debug_assert!(self.scheduled.is_empty() && !self.cleared);
        self.scheduled = results;
    }
}

impl Sequential for LoadQueue {
    fn finish_cycle(&mut self) {
        if self.cleared {
            self.queue.clear();
            self.scheduled = Vec::new();
            self.dequeued = false;
            self.cleared = false;
            return;
        }
        debug_assert!(
            self.queue.len() + self.scheduled.len() <= self.queue.capacity(),
            "load queue overflow"
        );

        if self.dequeued {
            self.queue.pop_front();
            self.dequeued = false;
        }
        self.queue.extend(mem::take(&mut self.scheduled));
    }
}

impl Clearable for LoadQueue {
    fn clear(&mut self) {
        debug_assert!(!self.cleared);
        self.cleared = true;
    }
}
