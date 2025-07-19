use super::{NotReady, Ready, ReorderBuffer, RobEntry, RobEntryHolder, RobIndex};

use std::mem;
use std::ops::Deref;

pub(in crate::components::cpu::exec_engine) struct RobLock<'a> {
    rob: &'a mut ReorderBuffer,
    issued: Vec<(RobIndex, RobEntryHolder)>,
    committed: Vec<RobIndex>,
}

//TODO Add a note mentioning that the destructor has to run to properly issue the instructions
impl<'a> RobLock<'a> {
    pub(super) fn new(rob: &'a mut ReorderBuffer) -> Self {
        Self {
            rob,
            issued: Vec::new(),
            committed: Vec::new(),
        }
    }

    /// Tries to reserve a new entry in the _ROB_.
    /// Returns [`None`] if the _ROB_ is full.
    pub fn reserve<'l>(&'l mut self) -> Option<RobEntryLock<'a, 'l>> {
        debug_assert!(
            self.committed.is_empty(),
            "cannot issue and commit with the same lock"
        );

        RobIndex::new(self.rob, self.issued.len()).map(|i| RobEntryLock(self, i))
    }

    pub fn pop_if_ready(&mut self) -> Option<RobEntry<Ready>> {
        debug_assert!(
            self.issued.is_empty(),
            "cannot issue and commit with the same lock"
        );
        let head = self.rob.head.read().add(self.rob, self.committed.len());
        match self.rob.get_mut(head).read() {
            RobEntryHolder::Empty | RobEntryHolder::NotReady(_) => None,
            RobEntryHolder::Ready(entry) => {
                self.committed.push(head);
                Some(*entry)
            }
        }
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
        debug_assert!(self.issued.is_empty() || self.committed.is_empty());

        // Issue
        if !self.issued.is_empty() {
            self.rob.len.push(self.issued.len());
            for (index, holder) in mem::take(&mut self.issued) {
                self.rob.get_mut(index).write(holder);
            }
        }

        // Commit
        if !self.committed.is_empty() {
            self.rob.len.pop(self.committed.len());
            let new_head = self.committed.last().unwrap().add(self.rob, 1);
            self.rob.head.write(new_head);
            for index in mem::take(&mut self.committed) {
                self.rob.get_mut(index).write(RobEntryHolder::Empty);
            }
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
        lock.issued.push((index, RobEntryHolder::Ready(entry)));
    }
}
