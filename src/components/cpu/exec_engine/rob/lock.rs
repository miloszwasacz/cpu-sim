use super::{NotReady, Ready, ReorderBuffer, RobEntry, RobEntryHolder, RobIndex};

use std::ops::Deref;

pub(in crate::components::cpu::exec_engine) struct RobLock<'a> {
    rob: &'a mut ReorderBuffer,
    issued: usize,
}

//TODO Add a note mentioning that the destructor HAS TO RUN to update the ROB length correctly
impl<'a> RobLock<'a> {
    pub(super) fn new(rob: &'a mut ReorderBuffer) -> Self {
        Self { rob, issued: 0 }
    }

    /// Tries to reserve a new entry in the _ROB_.
    /// Returns [`None`] if the _ROB_ is full.
    pub fn reserve<'l>(&'l mut self) -> Option<RobEntryLock<'a, 'l>> {
        RobIndex::new(self.rob, self.issued).map(|i| RobEntryLock(self, i))
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
        self.rob.len.push(self.issued);
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
        lock.rob
            .get_mut(index)
            .write(RobEntryHolder::NotReady(entry));
        lock.issued += 1;
    }

    pub fn issue_ready(self, entry: RobEntry<Ready>) {
        let Self(lock, index) = self;
        lock.rob.get_mut(index).write(RobEntryHolder::Ready(entry));
        lock.issued += 1;
    }
}
