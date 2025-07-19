pub(in crate::components::cpu) use self::lock::DecodeQueueLock;
use crate::components::cpu::error::Exception;
use crate::components::cpu::flip_flop::{Clearable, Sequential};
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::full::FullInstruction;
use crate::instr::Instruction;

use std::collections::VecDeque;
use std::mem;
use std::num::NonZeroUsize;

pub mod diagnostics;
mod lock;

pub struct DecodeQueue {
    queue: VecDeque<DecodeQueueEntry>,
    pushed: Vec<DecodeQueueEntry>,
    popped: usize,
    cleared: bool,
}

impl DecodeQueue {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity.get()),
            pushed: Default::default(),
            popped: Default::default(),
            cleared: Default::default(),
        }
    }

    pub(super) fn try_push(&mut self, decoded: impl IntoIterator<Item = DecodeQueueEntry>) -> bool {
        debug_assert!(self.pushed.is_empty());
        let instrs = decoded.into_iter().collect::<Vec<_>>();
        if self.queue.len() + instrs.len() > self.queue.capacity() {
            return false;
        }

        self.pushed.extend(instrs);
        true
    }

    pub(in crate::components::cpu) fn pop(&mut self) -> DecodeQueueLock {
        debug_assert_eq!(self.popped, 0);
        DecodeQueueLock::new(self)
    }
}

impl Sequential for DecodeQueue {
    fn finish_cycle(&mut self) {
        if self.cleared {
            self.queue.clear();
            self.pushed = Vec::new();
            self.popped = 0;
            self.cleared = false;
            return;
        }
        debug_assert!(
            self.queue.len() + self.pushed.len() <= self.queue.capacity(),
            "decode queue overflow"
        );

        drop(self.queue.drain(..self.popped));
        self.popped = 0;
        self.queue.extend(mem::take(&mut self.pushed));
    }
}

impl Clearable for DecodeQueue {
    fn clear(&mut self) {
        debug_assert!(!self.cleared);
        self.cleared = true;
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum DecodeQueueEntry {
    Ok(Decoded),
    Exception(Exception, Pc),
}

#[derive(Debug, Clone, Copy)]
pub struct Decoded {
    pub instr: Instruction,
    pub full: FullInstruction,
    pub pc: Pc,
    pub pc_plus_4: Pc,
    /// The predicted target address of a jump or a branch.
    pub predicted: Address,
    /// The pre-computed target address of pc-based jumps or branches.
    pub target: Address,
}
