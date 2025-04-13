use super::{NotReady, Ready, ReorderBuffer, RobEntry, RobEntryHolder, RobIndex};

pub(in crate::components::cpu::exec_engine) struct RobEntryLock<'a>(
    &'a mut ReorderBuffer,
    RobIndex,
);

impl<'a> RobEntryLock<'a> {
    pub(super) fn new(rob: &'a mut ReorderBuffer, index: RobIndex) -> Self {
        Self(rob, index)
    }

    pub fn index(&self) -> RobIndex {
        self.1
    }

    pub fn issue_not_ready(self, entry: RobEntry<NotReady>) {
        let Self(rob, index) = self;
        rob.get_mut(index).write(RobEntryHolder::NotReady(entry));
        rob.len.push();
    }

    pub fn issue_ready(self, entry: RobEntry<Ready>) {
        let Self(rob, index) = self;
        rob.get_mut(index).write(RobEntryHolder::Ready(entry));
        rob.len.push();
    }
}
