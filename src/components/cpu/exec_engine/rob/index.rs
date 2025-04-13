use super::{Item, ReorderBuffer};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RobIndex(usize);

impl RobIndex {
    /// Creates a new [`ReorderBuffer`] index pointing to the first empty
    /// space after the last element in the _ROB_.
    /// Returns [`None`] if the _ROB_ is full.
    ///
    /// # Panics
    ///
    /// This function will panic if the _ROB_'s capacity is zero.
    pub(super) fn new(rob: &ReorderBuffer) -> Option<Self> {
        let len = rob.len.read();
        if len == rob.capacity() {
            None
        } else {
            Some(rob.head.read().add(rob, len))
        }
    }

    /// Creates a new [`ReorderBuffer`] index pointing `rhs` positions further than `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if the _ROB_'s capacity is zero.
    #[inline(always)]
    pub(super) fn add(self, rob: &ReorderBuffer, rhs: usize) -> Self {
        Self(self.0.wrapping_add(rhs) % rob.capacity())
    }
}

impl PartialEq<usize> for RobIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for RobIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ReorderBuffer {
    pub(super) fn get(&self, index: RobIndex) -> &Item {
        &self.buffer[index.0]
    }

    pub(super) fn get_mut(&mut self, index: RobIndex) -> &mut Item {
        &mut self.buffer[index.0]
    }
}
