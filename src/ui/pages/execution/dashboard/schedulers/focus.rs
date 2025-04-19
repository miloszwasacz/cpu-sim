use crate::ui::Focus;

use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Selected {
    count: NonZeroUsize,
    focused: usize,
}

impl Selected {
    pub fn new(count: NonZeroUsize) -> Self {
        Self { count, focused: 0 }
    }

    pub fn get(&self) -> usize {
        self.focused
    }

    pub fn first(self) -> Self {
        Self { focused: 0, ..self }
    }

    pub fn last(self) -> Self {
        Self {
            focused: self.count.get() - 1,
            ..self
        }
    }
}

impl Focus for Selected {
    fn next(self) -> Self {
        let focused = self.focused;
        let next = if focused < self.count.get() - 1 {
            focused + 1
        } else {
            focused
        };
        Self {
            focused: next,
            ..self
        }
    }

    fn prev(self) -> Self {
        let focused = self.focused;
        let prev = focused.saturating_sub(1);
        Self {
            focused: prev,
            ..self
        }
    }
}
