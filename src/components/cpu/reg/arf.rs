use super::{RegData, RegFile, Register, ARCH_REG_COUNT};
use crate::instr::raw::{Bits, REG_LEN};

use std::fmt;

//#region Register File

#[derive(Debug, Clone, Copy)]
pub struct ArchRegFile([Register; ARCH_REG_COUNT]);

impl ArchRegFile {
    pub const SIZE: usize = ARCH_REG_COUNT;

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl Default for ArchRegFile {
    fn default() -> Self {
        Self::new()
    }
}

impl RegFile for ArchRegFile {
    type Index = ArchRegName;

    fn get(&self, reg: Self::Index) -> &Register {
        &self.0[reg.0]
    }

    fn set(&mut self, reg: Self::Index, data: RegData) {
        if reg.is_zero() {
            return;
        }

        self.0[reg.0].set(data);
    }
}

//#endregion

//#region Register Name

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArchRegName(usize);

impl ArchRegName {
    pub const ZERO: Self = Self(0);

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<Bits<REG_LEN>> for ArchRegName {
    fn from(value: Bits<REG_LEN>) -> Self {
        let name = u64::from(value) as usize;
        debug_assert!(name < ARCH_REG_COUNT, "x{} is not a valid register", name);
        Self(name)
    }
}

impl From<ArchRegName> for usize {
    fn from(value: ArchRegName) -> Self {
        value.0
    }
}

impl fmt::Display for ArchRegName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = if f.alternate() { 4 } else { 0 };
        write!(
            f,
            "{:>width$}",
            match self.0 {
                // Always zero
                0 => "zero",

                // Return address
                1 => "ra",

                // Stack pointer
                2 => "sp",

                // Global pointer
                3 => "gp",

                // Thread pointer
                4 => "tp",

                // Temporary / alternate return address
                5 => "t0",

                // Temporaries
                6 => "t1",
                7 => "t2",

                // Saved register / frame pointer
                8 => "s0",

                // Saved register
                9 => "s1",

                // Function arguments / return values
                10 => "a0",
                11 => "a1",

                // Function arguments
                12 => "a2",
                13 => "a3",
                14 => "a4",
                15 => "a5",
                16 => "a6",
                17 => "a7",

                // Saved registers
                18 => "s2",
                19 => "s3",
                20 => "s4",
                21 => "s5",
                22 => "s6",
                23 => "s7",
                24 => "s8",
                25 => "s9",
                26 => "s10",
                27 => "s11",

                // Temporaries
                28 => "t3",
                29 => "t4",
                30 => "t5",
                31 => "t6",

                _ => unreachable!(),
            }
        )
    }
}

//#endregion
