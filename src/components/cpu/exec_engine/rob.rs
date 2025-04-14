pub(super) use self::entry::{NotReady, Ready, ReadyRobEntry, RobEntry};
use self::flip_flop::RobLenFlipFlop;
pub use self::index::RobIndex;
use self::iter::Iter;
pub(super) use self::lock::RobEntryLock;
use super::exec_unit::ExecResult;
use crate::components::cpu::flip_flop::{Clearable, FlipFlop, Sequential};
use crate::components::memory::Address;
use crate::instr::EnvTrap;

use std::fmt;
use std::num::NonZeroUsize;

pub mod diagnostics;
mod entry;
mod flip_flop;
mod index;
mod iter;
mod lock;

//#region RobEntryHolder

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum RobEntryHolder {
    #[default]
    Empty,
    NotReady(RobEntry<NotReady>),
    Ready(RobEntry<Ready>),
}

impl FlipFlop<RobEntryHolder> {
    fn take_if_ready(&mut self) -> Option<RobEntry<Ready>> {
        match self.read() {
            RobEntryHolder::Empty | RobEntryHolder::NotReady(_) => None,
            RobEntryHolder::Ready(entry) => {
                let entry = *entry;
                self.write(RobEntryHolder::Empty);
                Some(entry)
            }
        }
    }
}

//#endregion

//#region ROB

type Item = FlipFlop<RobEntryHolder>;

pub struct ReorderBuffer {
    buffer: Box<[FlipFlop<RobEntryHolder>]>,
    head: FlipFlop<RobIndex>,
    len: RobLenFlipFlop,
}

impl ReorderBuffer {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        let buffer = vec![FlipFlop::new(RobEntryHolder::Empty); capacity.get()];
        Self {
            buffer: buffer.into_boxed_slice(),
            head: FlipFlop::new(Default::default()),
            len: Default::default(),
        }
    }

    #[inline(always)]
    fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Tries to reserve a new entry in the _ROB_.
    /// Returns [`None`] if the _ROB_ is full.
    pub(super) fn reserve(&mut self) -> Option<RobEntryLock> {
        RobIndex::new(self).map(|i| RobEntryLock::new(self, i))
    }

    pub fn pop_if_ready(&mut self) -> Option<(RobIndex, RobEntry<Ready>)> {
        let head = *self.head.read();
        self.get_mut(head).take_if_ready().map(|entry| {
            self.head.write(head.add(self, 1));
            self.len.pop();
            (head, entry)
        })
    }

    pub fn update_from_cdb(&mut self, results: &[ExecResult]) {
        for result in results {
            let holder = self.get_mut(result.tag);
            let current = *holder.read();
            let new = match current {
                RobEntryHolder::NotReady(not_ready) => match not_ready.update(result) {
                    Ok(ready) => RobEntryHolder::Ready(ready),
                    Err(not_ready) => RobEntryHolder::NotReady(not_ready),
                },
                RobEntryHolder::Empty => unreachable!("instruction already commited"),
                RobEntryHolder::Ready(_) => unreachable!("instruction executed more than once"),
            };

            if new != current {
                holder.write(new);
            }
        }
    }

    pub fn preceding_stores(
        &self,
        index: RobIndex,
    ) -> impl Iterator<Item = Result<StoreMeta, EnvTrap>> {
        let head = *self.head.read();
        Iter::new(self, head..index).filter_map(|h| match h.read() {
            RobEntryHolder::Empty => None,
            RobEntryHolder::NotReady(entry) => match entry.data() {
                NotReady::Store { byte_count, .. } => Some(Ok(StoreMeta {
                    byte_count: *byte_count,
                    addr: None,
                })),
                _ => None,
            },
            RobEntryHolder::Ready(entry) => match entry.data() {
                Ok(ReadyRobEntry::Store {
                    byte_count, addr, ..
                }) => Some(Ok(StoreMeta {
                    byte_count: *byte_count,
                    addr: Some(*addr),
                })),
                Ok(_) => None,
                Err(trap) => Some(Err(*trap)),
            },
        })
    }

    pub(super) fn get_if_ready(&self, index: RobIndex) -> Option<&RobEntry<Ready>> {
        match self.get(index).read() {
            RobEntryHolder::Ready(ready) => Some(ready),
            RobEntryHolder::NotReady(_) | RobEntryHolder::Empty => None,
        }
    }
}

impl Sequential for ReorderBuffer {
    fn finish_cycle(&mut self) {
        for entry in &mut self.buffer {
            entry.finish_cycle();
        }
        self.head.finish_cycle();
        self.len.finish_cycle();
    }
}

impl Clearable for ReorderBuffer {
    fn clear(&mut self) {
        for entry in &mut self.buffer {
            entry.clear();
        }
        self.head.clear();
        self.len.clear();
    }
}

impl fmt::Display for ReorderBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ROB:")?;
        let start = *self.head.read();
        let mut i = start;
        loop {
            writeln!(f, "{:2}: {:?}", i, self.get(i).read())?;
            i = i.add(self, 1);
            if i == start {
                break;
            }
        }
        Ok(())
    }
}

//#endregion

/// Simple metadata about store instructions
pub struct StoreMeta {
    pub byte_count: usize,
    pub addr: Option<Address>,
}
