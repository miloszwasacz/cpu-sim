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
    dequeued: Option<usize>,
    cleared: bool,
}

impl LoadQueue {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        LoadQueue {
            queue: VecDeque::with_capacity(capacity.get()),
            scheduled: Vec::new(),
            dequeued: None,
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
            .enumerate()
            .find(|(_, entry)| Self::is_entry_ready(entry, rob))
            .map(|(i, instr)| {
                self.dequeued = Some(i);
                instr.execute(mem)
            })
    }

    pub fn write(&mut self, results: Vec<LoadQueueEntry>) {
        debug_assert!(self.scheduled.is_empty() && !self.cleared);
        self.scheduled = results;
    }

    fn is_entry_ready(entry: &LoadQueueEntry, rob: &ReorderBuffer) -> bool {
        // There can be no RAW hazards caused by preceding stores or traps
        !rob.preceding_stores(entry.tag())
            .map_ok(|store| entry.has_hazard(store))
            .any(|hazard| hazard.unwrap_or(true))
    }
}

impl Sequential for LoadQueue {
    fn finish_cycle(&mut self) {
        if self.cleared {
            self.queue.clear();
            self.scheduled = Vec::new();
            self.dequeued = None;
            self.cleared = false;
            return;
        }
        debug_assert!(
            self.queue.len() + self.scheduled.len() <= self.queue.capacity(),
            "load queue overflow"
        );

        if let Some(i) = self.dequeued {
            self.queue.remove(i);
            self.dequeued = None;
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
