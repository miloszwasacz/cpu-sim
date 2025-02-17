use super::{RegData, RegFile, Register, ARCH_REG_COUNT};
use crate::instr::raw::{Bits, REG_LEN};

use cpu_sim_derive::register_names;
use std::error::Error;
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

impl fmt::Display for ArchRegFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registers:")?;
        for (name, reg) in self.0.iter().enumerate() {
            let name: ArchRegName = name.try_into().unwrap();
            writeln!(f, "  {:#}: {}", name, reg.get())?;
        }
        Ok(())
    }
}

//#endregion

//#region Register Name

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArchRegName(usize);

register_names!(ArchRegName {
    // Always zero
    reg ZERO = 0,

    // Return address
    reg RA = 1,

    // Stack pointer
    reg SP = 2,

    // Global pointer
    reg GP = 3,

    // Thread pointer
    reg TP = 4,

    // Temporary / alternate return address
    reg T0 = 5,

    // Temporaries
    reg T1 = 6,
    reg T2 = 7,

    // Saved register / frame pointer
    reg S0 = 8,

    // Saved register
    reg S1 = 9,

    // Function arguments / return values
    reg A0 = 10,
    reg A1 = 11,

    // Function arguments
    reg A2 = 12,
    reg A3 = 13,
    reg A4 = 14,
    reg A5 = 15,
    reg A6 = 16,
    reg A7 = 17,

    // Saved registers
    reg S2 = 18,
    reg S3 = 19,
    reg S4 = 20,
    reg S5 = 21,
    reg S6 = 22,
    reg S7 = 23,
    reg S8 = 24,
    reg S9 = 25,
    reg S10 = 26,
    reg S11 = 27,

    // Temporaries
    reg T3 = 28,
    reg T4 = 29,
    reg T5 = 30,
    reg T6 = 31,
});

impl ArchRegName {
    pub const FP: Self = Self::S0;

    pub const fn is_zero(&self) -> bool {
        self.0 == Self::ZERO.0
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

impl TryFrom<usize> for ArchRegName {
    type Error = ArchRegNameConvertError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < ARCH_REG_COUNT {
            Ok(Self(value))
        } else {
            Err(ArchRegNameConvertError(value))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchRegNameConvertError(usize);

impl fmt::Display for ArchRegNameConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{} is not a valid register", self.0)
    }
}

impl Error for ArchRegNameConvertError {}

//#endregion
