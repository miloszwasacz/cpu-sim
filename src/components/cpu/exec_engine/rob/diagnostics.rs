use super::entry::RobEntryData;
pub use super::entry::{NotReady, Ready};
use super::{ReorderBuffer, RobEntryHolder, RobIndex};
use crate::components::diagnostics::Diagnostics;
use crate::components::memory::Address;

pub struct RobSnapshot {
    pub entries: Box<[(RobIndex, RobEntrySnapshot)]>,
}

#[allow(private_bounds)]
pub struct RobEntry<D: RobEntryData> {
    pub addr: Address,
    pub data: D,
}

pub enum RobEntrySnapshot {
    Empty,
    NotReady(RobEntry<NotReady>),
    Ready(RobEntry<Ready>),
}

impl From<RobEntryHolder> for RobEntrySnapshot {
    fn from(value: RobEntryHolder) -> Self {
        match value {
            RobEntryHolder::Empty => Self::Empty,
            RobEntryHolder::NotReady(entry) => Self::NotReady(entry.into()),
            RobEntryHolder::Ready(entry) => Self::Ready(entry.into()),
        }
    }
}

impl Diagnostics for ReorderBuffer {
    type Output = RobSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let mut entries = Vec::with_capacity(self.buffer.len());

        let mut i = *self.head.read();
        while entries.len() < self.buffer.len() {
            let entry = *self.get(i).read();
            entries.push((i, entry.into()));
            i = i.add(self, 1);
        }

        let entries = entries.into_boxed_slice();
        Self::Output { entries }
    }
}
