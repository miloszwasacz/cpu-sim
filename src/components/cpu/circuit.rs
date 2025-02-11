use std::any::type_name_of_val;
use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockCycle {
    FirstHalf,
    SecondHalf,
    Full,
}

#[derive(Debug)]
pub struct Circuit<T> {
    /// Whether the element is being read during the first half of the clock cycle.
    #[cfg(debug_assertions)]
    read1: Cell<bool>,
    /// Whether the element is being read during the second half of the clock cycle.
    #[cfg(debug_assertions)]
    read2: Cell<bool>,
    /// Whether the element is being written during the first half of the clock cycle.
    #[cfg(debug_assertions)]
    write1: bool,
    /// Whether the element is being written during the second half of the clock cycle.
    #[cfg(debug_assertions)]
    write2: bool,
    /// A part of the CPU.
    element: T,
}

impl<T> Circuit<T> {
    pub fn new(element: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            read1: Cell::new(Default::default()),
            #[cfg(debug_assertions)]
            read2: Cell::new(Default::default()),
            #[cfg(debug_assertions)]
            write1: Default::default(),
            #[cfg(debug_assertions)]
            write2: Default::default(),
            element,
        }
    }

    pub fn read(&self, cycle_half: ClockCycle) -> &T {
        #[cfg(debug_assertions)]
        {
            let has_error = match cycle_half {
                ClockCycle::FirstHalf => self.write1,
                ClockCycle::SecondHalf => self.write2,
                ClockCycle::Full => self.write1 || self.write2,
            };
            if has_error {
                panic!(
                    "cannot read: {} is being written",
                    type_name_of_val(&self.element)
                );
            }

            match cycle_half {
                ClockCycle::FirstHalf => self.read1.set(true),
                ClockCycle::SecondHalf => self.read2.set(true),
                ClockCycle::Full => {
                    self.read1.set(true);
                    self.read2.set(true);
                }
            }
        }
        &self.element
    }

    pub fn write(&mut self, cycle_half: ClockCycle) -> &mut T {
        #[cfg(debug_assertions)]
        {
            let has_read_error = match cycle_half {
                ClockCycle::FirstHalf => self.read1.get(),
                ClockCycle::SecondHalf => self.read2.get(),
                ClockCycle::Full => self.read1.get() || self.read2.get(),
            };
            if has_read_error {
                panic!(
                    "cannot write: {} is being read",
                    type_name_of_val(&self.element)
                );
            }
            let has_write_error = match cycle_half {
                ClockCycle::FirstHalf => self.write1,
                ClockCycle::SecondHalf => self.write2,
                ClockCycle::Full => self.write1 || self.write2,
            };
            if has_write_error {
                panic!(
                    "cannot write: {} is being written",
                    type_name_of_val(&self.element)
                );
            }

            match cycle_half {
                ClockCycle::FirstHalf => self.write1 = true,
                ClockCycle::SecondHalf => self.write2 = true,
                ClockCycle::Full => {
                    self.write1 = true;
                    self.write2 = true;
                }
            }
        }
        &mut self.element
    }

    pub fn reset(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.read1.set(Default::default());
            self.read2.set(Default::default());
            self.write1 = Default::default();
            self.write2 = Default::default();
        }
    }
    
    pub unsafe fn inner(&self) -> &T {
        &self.element
    } 
    
    pub unsafe fn inner_mut(&mut self) -> &mut T {
        &mut self.element
    }
}

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
