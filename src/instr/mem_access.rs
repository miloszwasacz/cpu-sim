use crate::components::cpu::reg::RegData;
use crate::components::memory::{Address, L1D};

pub type MemRead = fn(&mut L1D, Address) -> RegData;
pub type MemWrite = fn(&mut L1D, Address, RegData);
