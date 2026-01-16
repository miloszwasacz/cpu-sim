//! Type definitions for functions accessing the L1 data cache.

use crate::components::cpu::reg::RegData;
use crate::components::memory::{Address, L1D};

/// A function that can read data from an L1 data cache.
pub type MemRead = fn(&mut L1D, Address) -> RegData;

/// A function that can write data to an L1 data cache.
pub type MemWrite = fn(&mut L1D, Address, RegData);
