use super::{ReorderBuffer, RobEntryHolder, RobIndex};
use crate::components::cpu::flip_flop::FlipFlop;

use std::ops::Range;

pub(super) struct Iter<'a> {
    rob: &'a ReorderBuffer,
    index: Range<RobIndex>,
}

impl<'a> Iter<'a> {
    pub fn new(rob: &'a ReorderBuffer, range: Range<RobIndex>) -> Self {
        Self { rob, index: range }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a FlipFlop<RobEntryHolder>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.start == self.index.end {
            return None;
        }

        let index = self.index.start;
        self.index.start = self.index.start.add(self.rob, 1);
        Some(self.rob.get(index))
    }
}
