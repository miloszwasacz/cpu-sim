use crate::instr::raw::Bits;
use crate::instr::decode::REG_LEN;

use std::fmt;

pub type RegSize = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterName {
    Zero,
    X(GprId),
}

pub type GprId = u8;

impl RegisterName {
    pub const MAX_GPR: u64 = (1 << REG_LEN) - 2;

    pub fn decode(encoded: Bits<REG_LEN>) -> Self {
        let encoded = u64::from(encoded);
        match encoded {
            0 => RegisterName::Zero,
            #[cfg(debug_assertions)]
            r if r > Self::MAX_GPR => panic!("x{r} is not a valid register"),
            r => RegisterName::X(r as GprId),
        }
    }
}

impl fmt::Display for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = if f.alternate() { 4 } else { 0 };
        let name = match self {
            RegisterName::Zero => 0,
            RegisterName::X(name) => *name,
        };
        // println!("{:?}", f.alternate());
        write!(
            f,
            "{:>width$}",
            match name {
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
