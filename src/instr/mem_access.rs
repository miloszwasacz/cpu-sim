use crate::components::cpu::reg::RegData;
use crate::components::memory::{Address, Memory};

pub type MemRead = fn(&Memory, Address) -> RegData;
pub type MemWrite = fn(&mut Memory, Address, RegData);
