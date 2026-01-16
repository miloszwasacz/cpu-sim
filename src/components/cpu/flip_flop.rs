use std::fmt;

pub trait Sequential {
    fn finish_cycle(&mut self);
}

pub trait Clearable: Sequential {
    fn clear(&mut self);
}

//#region FlipFlop

/// A clock-cycle aware circuit for storing data.
#[derive(Debug, Clone, Copy)]
pub struct FlipFlop<T> {
    value: T,
    new: Option<T>,
    cleared: bool,
}

impl<T> FlipFlop<T> {
    /// Creates a new flip-flop with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            value: initial,
            new: None,
            cleared: false,
        }
    }

    /// Reads the currently stored value.
    ///
    /// Note that [writing](Self::write) a new value will not change what
    /// is returned by this method until [`Sequential::finish_cycle`] has been called.
    pub fn read(&self) -> &T {
        &self.value
    }

    /// Stores a new value.
    ///
    /// Note that this will not change what is returned by [`Self::read`] until
    /// [`Sequential::finish_cycle`] has been called.
    ///
    /// # Panics
    ///
    /// When `debug_assertions` are enabled, panics if the circuit has already
    /// been written to or cleared within one clock cycle.
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

/// A [`FlipFlop`] that can be stalled.
#[derive(Debug, Clone, Copy)]
pub struct StallingFlipFlop<T> {
    flip_flop: FlipFlop<T>,
    stalled: Option<T>,
    stalling: bool,
}

impl<T> StallingFlipFlop<T> {
    /// Creates a new stalling flip-flop with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            flip_flop: FlipFlop::new(initial),
            stalled: None,
            stalling: false,
        }
    }

    /// Reads the currently stored value.
    ///
    /// Note that [writing](Self::write) a new value will not change what
    /// is returned by this method until [`Sequential::finish_cycle`] has been called.
    pub fn read(&self) -> &T {
        self.flip_flop.read()
    }

    /// Stores a new value.
    ///
    /// If the circuit has been stalled
    ///
    /// Note that this will not change what is returned by [`Self::read`] until
    /// [`Sequential::finish_cycle`] has been called.
    ///
    /// # Panics
    ///
    /// When `debug_assertions` are enabled, panics if the circuit has already
    /// been written to, cleared, or stalled within one clock cycle.
    pub fn write(&mut self, new: T) {
        debug_assert!(!self.stalling, "cannot write after stalling");
        self.flip_flop.write(new);
    }

    /// Stalls the circuit.
    ///
    /// # Example
    ///
    /// ```
    /// # use cpu_sim::components::cpu::flip_flop::{StallingFlipFlop, Sequential};
    ///
    /// let mut buf = StallingFlipFlop::new(1);
    /// assert_eq!(*buf.read(), 1);
    ///
    /// // Writing changes the value for the next clock cycle.
    /// buf.write(2);
    /// buf.finish_cycle();
    /// assert_eq!(*buf.read(), 2);
    ///
    /// // Stalling preserves the original value.
    /// buf.write(3);
    /// buf.stall();
    /// buf.finish_cycle();
    /// assert_eq!(*buf.read(), 2);
    ///
    /// // Writing after stalling changes the value.
    /// buf.write(4);
    /// buf.finish_cycle();
    /// assert_eq!(*buf.read(), 4);
    ///
    /// // Writing, stalling, and not writing again in the next cycle
    /// // changes the value to the previously written value.
    /// buf.write(5);
    /// buf.stall();
    /// buf.finish_cycle();
    /// assert_eq!(*buf.read(), 4);
    /// buf.finish_cycle();
    /// assert_eq!(*buf.read(), 5);
    /// ```
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

        let stalled = self.stalled.take();
        self.flip_flop.new = self.flip_flop.new.take().or(stalled);
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
