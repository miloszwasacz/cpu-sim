use super::{Item, ReorderBuffer};

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RobIndex(usize);

impl RobIndex {
    /// Creates a new [`ReorderBuffer`] index pointing to the first empty
    /// space after the last element in the _ROB_ + `offset`.
    /// Returns [`None`] if the _ROB_ is full.
    ///
    /// # Panics
    ///
    /// This function will panic if the _ROB_'s capacity is zero.
    pub(super) fn new(rob: &ReorderBuffer, offset: usize) -> Option<Self> {
        let len = rob.len.read() + offset;
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

    pub fn cmp(&self, rob: &ReorderBuffer, rhs: &RobIndex) -> Ordering {
        let head = rob.head.read();
        let lb = self.0 >= head.0; // `self` is right of `head`
        let rb = rhs.0 >= head.0; // `rhs` is right of `head`
        if lb && !rb {
            Ordering::Less
        } else if !lb && rb {
            Ordering::Greater
        } else {
            self.0.cmp(&rhs.0)
        }
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
