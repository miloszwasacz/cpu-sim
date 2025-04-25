use crate::components::cpu::flip_flop::{Clearable, Sequential};

//#region RobLenFlipFlop

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

//#endregion

//#region RobTrapFlipFlop

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RobTrapFlipFlop {
    value: bool,
    pushed: bool,
    popped: bool,
    cleared: bool,
}

impl RobTrapFlipFlop {
    #[inline]
    pub fn read(&self) -> bool {
        self.value
    }

    pub fn push(&mut self) {
        debug_assert!(!self.pushed && !self.cleared);
        self.pushed = true;
    }

    pub fn pop(&mut self) {
        debug_assert!(!self.popped && !self.cleared);
        self.popped = true;
    }
}

impl Sequential for RobTrapFlipFlop {
    fn finish_cycle(&mut self) {
        let value = if self.cleared {
            Default::default()
        } else {
            (!self.popped && self.value) || self.pushed
        };
        *self = Self {
            value,
            ..Default::default()
        }
    }
}

impl Clearable for RobTrapFlipFlop {
    fn clear(&mut self) {
        self.cleared = true;
    }
}

//#endregion
