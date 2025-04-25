use super::{NotReady, Ready, ReorderBuffer, RobEntry, RobEntryHolder, RobIndex};

use std::mem;
use std::ops::Deref;

pub(in crate::components::cpu::exec_engine) struct RobLock<'a> {
    rob: &'a mut ReorderBuffer,
    issued: Vec<(RobIndex, RobEntryHolder)>,
    has_trap: bool,
}

//TODO Add a note mentioning that the destructor has to run to properly issue the instructions
impl<'a> RobLock<'a> {
    pub(super) fn new(rob: &'a mut ReorderBuffer) -> Self {
        Self {
            rob,
            issued: Vec::new(),
            has_trap: false,
        }
    }

    /// Tries to reserve a new entry in the _ROB_.
    /// Returns [`None`] if the _ROB_ is full.
    pub fn reserve<'l>(&'l mut self) -> Option<RobEntryLock<'a, 'l>> {
        if self.rob.has_trap.read() {
            return None;
        }

        RobIndex::new(self.rob, self.issued.len()).map(|i| RobEntryLock(self, i))
    }
}

impl Deref for RobLock<'_> {
    type Target = ReorderBuffer;

    fn deref(&self) -> &Self::Target {
        self.rob
    }
}

impl Drop for RobLock<'_> {
    fn drop(&mut self) {
        self.rob.len.push(self.issued.len());
        for (index, holder) in mem::take(&mut self.issued) {
            self.rob.get_mut(index).write(holder);
        }
        if self.has_trap {
            self.rob.has_trap.push();
        }
    }
}

pub(in crate::components::cpu::exec_engine) struct RobEntryLock<'a, 'l>(
    &'l mut RobLock<'a>,
    RobIndex,
);

impl RobEntryLock<'_, '_> {
    pub fn index(&self) -> RobIndex {
        self.1
    }

    pub fn issue_not_ready(self, entry: RobEntry<NotReady>) {
        let Self(lock, index) = self;
        lock.issued.push((index, RobEntryHolder::NotReady(entry)));
    }

    pub fn issue_ready(self, entry: RobEntry<Ready>) {
        let Self(lock, index) = self;
        if entry.data().is_err() {
            debug_assert!(!lock.has_trap);
            lock.has_trap = true;
        }
        lock.issued.push((index, RobEntryHolder::Ready(entry)));
    }
}
