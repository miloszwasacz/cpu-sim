pub(super) use self::entry::{NotReady, Ready, ReadyRobEntry, RobEntry};
use self::flip_flop::RobLenFlipFlop;
pub use self::index::RobIndex;
use self::iter::Iter;
pub(super) use self::lock::RobLock;
use super::exec_unit::ExecResult;
use crate::components::cpu::flip_flop::{Clearable, FlipFlop, Sequential};
use crate::components::cpu::reg::FutureFile;
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

    pub(super) fn lock(&mut self) -> RobLock {
        RobLock::new(self)
    }

    pub fn pop_if_ready(&mut self) -> Option<RobEntry<Ready>> {
        let head = *self.head.read();
        self.get_mut(head).take_if_ready().inspect(|_| {
            self.head.write(head.add(self, 1));
            self.len.pop();
        })
    }

    pub fn update_from_cdb(&mut self, future_file: &mut FutureFile, results: &[ExecResult]) {
        for result in results {
            let holder = self.get_mut(result.tag);
            let ready = match holder.read() {
                RobEntryHolder::NotReady(not_ready) => not_ready.update(result),
                RobEntryHolder::Empty => unreachable!("instruction already commited"),
                RobEntryHolder::Ready(_) => unreachable!("instruction executed more than once"),
            };
            //TODO If the instruction produces an exception,
            //     the destination register in the future file will not be freed
            let _ = ready.data().inspect(|data| match data {
                ReadyRobEntry::Alu { dest, value }
                | ReadyRobEntry::Load { dest, value }
                | ReadyRobEntry::Jump {
                    link_reg: dest,
                    link_data: value,
                    ..
                } => future_file[*dest].write_result(result.tag, *value),
                ReadyRobEntry::Branch { .. } | ReadyRobEntry::Store { .. } => {}
            });
            holder.write(RobEntryHolder::Ready(ready));
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
