use crate::components::cpu::flip_flop::{Clearable, Sequential};

//#region RobLenFlipFlop

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RobLenFlipFlop {
    value: usize,
    pushed: usize,
    popped: usize,
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

    pub fn pop(&mut self, n: usize) {
        debug_assert!(self.popped == 0 && !self.cleared);
        self.popped = n;
    }
}

impl Sequential for RobLenFlipFlop {
    fn finish_cycle(&mut self) {
        let value = if self.cleared {
            Default::default()
        } else {
            self.value - self.popped + self.pushed
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

//#endregion
