use crate::components::cpu::flip_flop::{Clearable, Sequential};

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RobLenFlipFlop {
    value: usize,
    pushed: usize,
    popped: bool,
    cleared: bool,
}

impl RobLenFlipFlop {
    #[inline]
    pub fn read(&self) -> usize {
        self.value
    }

    pub fn push(&mut self, n: usize) {
        debug_assert!(self.pushed == 0 && !self.cleared);
        self.pushed = n;
    }

    pub fn pop(&mut self) {
        debug_assert!(!self.popped && !self.cleared);
        self.popped = true;
    }
}

impl Sequential for RobLenFlipFlop {
    fn finish_cycle(&mut self) {
        let value = if self.cleared {
            Default::default()
        } else {
            let value = if self.popped {
                self.value - 1
            } else {
                self.value
            };
            value + self.pushed
        };
        *self = Self {
            value,
            ..Default::default()
        }
    }
}

impl Clearable for RobLenFlipFlop {
    fn clear(&mut self) {
        self.cleared = true;
    }
}
