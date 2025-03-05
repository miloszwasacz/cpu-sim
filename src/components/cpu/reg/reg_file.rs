use super::{Register, ARCH_REG_COUNT};
use super::name::RegName;
use super::data::RegData;

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct RegFile([Register; ARCH_REG_COUNT]);

impl RegFile {
    pub const SIZE: usize = ARCH_REG_COUNT;

    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn get(&self, reg: RegName) -> RegData {
        self.0[reg.0].get()
    }

    pub fn set(&mut self, reg: RegName, data: RegData) {
        if reg.is_zero() {
            return;
        }

        self.0[reg.0].set(data);
    }
}

impl Default for RegFile {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RegFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registers:")?;
        for (name, reg) in self.0.iter().enumerate() {
            const HEX_WIDTH: usize = size_of::<RegData>() * 2;
            let name: RegName = name.try_into().unwrap();
            let data = reg.get().i();
            writeln!(
                f,
                "  {name:#}: {data} ({data:#0width$x})",
                width = HEX_WIDTH
            )?;
        }
        Ok(())
    }
}
