use std::any::type_name_of_val;
use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockCycle {
    FirstHalf,
    SecondHalf,
}

#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct Circuit<T> {
    /// Whether the element is being read during the first half of the clock cycle.
    read1: Cell<bool>,
    /// Whether the element is being read during the second half of the clock cycle.
    read2: Cell<bool>,
    /// Whether the element is being written during the first half of the clock cycle.
    write1: Cell<bool>,
    /// Whether the element is being written during the second half of the clock cycle.
    write2: Cell<bool>,
    /// A part of the CPU.
    element: T,
}

#[cfg(not(debug_assertions))]
#[derive(Debug)]
pub struct Circuit<T> {
    /// A part of the CPU.
    element: T,
}

impl<T> Circuit<T> {
    #[cfg(debug_assertions)]
    pub fn new(element: T) -> Self {
        Self {
            read1: Default::default(),
            read2: Default::default(),
            write1: Default::default(),
            write2: Default::default(),
            element,
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn new(element: T) -> Self {
        Self { element }
    }

    #[inline]
    pub fn read(&self, cycle_half: ClockCycle) -> &T {
        #[cfg(debug_assertions)]
        self.read_check(cycle_half);
        &self.element
    }

    #[inline]
    pub fn write(&mut self, cycle_half: ClockCycle) -> &mut T {
        #[cfg(debug_assertions)]
        self.write_check(cycle_half);
        &mut self.element
    }

    #[inline]
    pub fn reset(&self) {
        #[cfg(debug_assertions)]
        {
            self.read1.take();
            self.read2.take();
            self.write1.take();
            self.write2.take();
        }
    }

    #[inline(always)]
    pub unsafe fn inner_mut(&mut self) -> &mut T {
        &mut self.element
    }
}

impl<T> Circuit<Cell<T>> {
    #[inline]
    pub fn read_cell(&self, cycle_half: ClockCycle) -> T
    where
        T: Copy,
    {
        #[cfg(debug_assertions)]
        self.read_check(cycle_half);
        self.element.get()
    }

    #[inline]
    pub fn write_cell(&self, cycle_half: ClockCycle, element: T) {
        #[cfg(debug_assertions)]
        self.write_check(cycle_half);
        self.element.set(element)
    }
}

//#region Checks

#[cfg(debug_assertions)]
impl<T> Circuit<T> {
    fn read_check(&self, cycle_half: ClockCycle) {
        if self.writes(cycle_half) {
            panic!(
                "cannot read: {} is being written",
                type_name_of_val(&self.element)
            );
        }

        match cycle_half {
            ClockCycle::FirstHalf => self.read1.set(true),
            ClockCycle::SecondHalf => self.read2.set(true),
        }
    }

    fn write_check(&self, cycle_half: ClockCycle) {
        if self.reads(cycle_half) {
            panic!(
                "cannot write: {} is being read",
                type_name_of_val(&self.element)
            );
        }
        if self.writes(cycle_half) {
            panic!(
                "cannot write: {} is being written",
                type_name_of_val(&self.element)
            );
        }

        match cycle_half {
            ClockCycle::FirstHalf => self.write1.set(true),
            ClockCycle::SecondHalf => self.write2.set(true),
        }
    }

    fn reads(&self, cycle_half: ClockCycle) -> bool {
        match cycle_half {
            ClockCycle::FirstHalf => self.read1.get(),
            ClockCycle::SecondHalf => self.read2.get(),
        }
    }

    fn writes(&self, cycle_half: ClockCycle) -> bool {
        match cycle_half {
            ClockCycle::FirstHalf => self.write1.get(),
            ClockCycle::SecondHalf => self.write2.get(),
        }
    }
}

//#endregion

impl<T: Default> Default for Circuit<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for Circuit<T> {
    fn from(value: T) -> Self {
        Circuit::new(value)
    }
}
