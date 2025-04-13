use std::fmt;

pub trait Sequential {
    fn finish_cycle(&mut self);
}

pub trait Clearable: Sequential {
    fn clear(&mut self);
}

//#region FlipFlop

#[derive(Debug, Clone, Copy)]
pub struct FlipFlop<T> {
    value: T,
    new: Option<T>,
    cleared: bool,
}

impl<T> FlipFlop<T> {
    pub fn new(circ: T) -> Self {
        Self {
            value: circ,
            new: None,
            cleared: false,
        }
    }

    pub fn read(&self) -> &T {
        &self.value
    }

    pub fn write(&mut self, new: T) {
        debug_assert!(self.new.is_none() && !self.cleared);
        self.new = Some(new);
    }
}

impl<T: Default> Sequential for FlipFlop<T> {
    fn finish_cycle(&mut self) {
        if let Some(new) = self.new.take() {
            self.value = new;
        }
        if self.cleared {
            self.value = T::default();
            self.cleared = false;
        }
    }
}

impl<T: Default> Clearable for FlipFlop<T> {
    fn clear(&mut self) {
        self.cleared = true;
    }
}

impl<T: fmt::Display> fmt::Display for FlipFlop<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

//#endregion

//#region StallingFlipFlop

#[derive(Debug, Clone, Copy)]
pub struct StallingFlipFlop<T> {
    flip_flop: FlipFlop<T>,
    stalled: Option<T>,
    stalling: bool,
}

impl<T> StallingFlipFlop<T> {
    pub fn new(circ: T) -> Self {
        Self {
            flip_flop: FlipFlop::new(circ),
            stalled: None,
            stalling: false,
        }
    }

    pub fn read(&self) -> &T {
        self.flip_flop.read()
    }

    pub fn write(&mut self, new: T) {
        debug_assert!(!self.stalling, "cannot write after stalling");
        self.flip_flop.write(new);
    }

    pub fn stall(&mut self) {
        debug_assert!(!self.stalling);
        self.stalling = true;
        self.stalled = self.flip_flop.new.take();
    }
}

impl<T: Default> Sequential for StallingFlipFlop<T> {
    fn finish_cycle(&mut self) {
        if self.stalling {
            self.stalling = false;
            return;
        }

        self.flip_flop.new = self.flip_flop.new.take().or(self.stalled.take());
        self.flip_flop.finish_cycle();
    }
}

impl<T: Default> Clearable for StallingFlipFlop<T> {
    fn clear(&mut self) {
        debug_assert!(!self.stalling, "cannot clear after stalling");
        self.flip_flop.clear();
    }
}

impl<T: fmt::Display> fmt::Display for StallingFlipFlop<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.flip_flop, f)
    }
}

//#endregion
