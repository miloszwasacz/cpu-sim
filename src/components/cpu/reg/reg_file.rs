use super::data::RegData;
use super::name::RegName;
use super::{Register, ARCH_REG_COUNT};
use crate::components::cpu::flip_flop::{Clearable, Sequential};
use crate::components::cpu::RobIndex;

use std::fmt;
use std::ops::{Index, IndexMut};

//#region RegFile

#[derive(Debug, Clone, Default)]
pub struct RegFile([Register; ARCH_REG_COUNT], RegStat);

impl RegFile {
    pub const SIZE: usize = ARCH_REG_COUNT;

    pub fn new() -> Self {
        Default::default()
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

    pub fn stat(&self) -> &RegStat {
        &self.1
    }

    pub fn stat_mut(&mut self) -> &mut RegStat {
        &mut self.1
    }
}

impl Sequential for RegFile {
    fn finish_cycle(&mut self) {
        for reg in &mut self.0 {
            reg.finish_cycle();
        }
        self.1.finish_cycle();
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

//#endregion

//#region RegStat

#[derive(Debug, Clone)]
pub struct RegStat([RegStatEntry; ARCH_REG_COUNT]);

impl Default for RegStat {
    fn default() -> Self {
        let mut stat = Self(Default::default());
        stat[RegName::ZERO] = RegStatEntry::zero();
        stat
    }
}

impl Index<RegName> for RegStat {
    type Output = RegStatEntry;

    fn index(&self, index: RegName) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<RegName> for RegStat {
    fn index_mut(&mut self, index: RegName) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl Sequential for RegStat {
    fn finish_cycle(&mut self) {
        for reg in &mut self.0 {
            reg.finish_cycle();
        }
    }
}

impl Clearable for RegStat {
    fn clear(&mut self) {
        for reg in &mut self.0 {
            reg.clear();
        }
    }
}

impl fmt::Display for RegStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RegStat:")?;
        for (name, reg) in self.0.iter().enumerate() {
            let name = RegName::try_from(name).unwrap();
            let data = reg.read();
            write!(f, "  {:#}: ", name)?;
            match data {
                None => writeln!(f, "-")?,
                Some(index) => writeln!(f, "{}", index)?,
            }
        }
        Ok(())
    }
}

//#endregion

//#region RegStatEntry

#[derive(Debug, Clone, Copy, Default)]
pub struct RegStatEntry {
    value: Option<RobIndex>,
    issued: Option<RobIndex>,
    committed: Option<RobIndex>,
    cleared: bool,
    /// Whether this entry is hardwired to the ZERO register
    zero: bool,
}

impl RegStatEntry {
    fn zero() -> Self {
        Self {
            zero: true,
            ..Default::default()
        }
    }

    pub fn read(&self) -> Option<RobIndex> {
        if self.zero {
            debug_assert!(self.value.is_none());
            return None;
        }

        self.value
    }

    pub fn issue_new(&mut self, index: RobIndex) {
        debug_assert!(self.issued.is_none() && !self.cleared);

        if !self.zero {
            self.issued = Some(index);
        }
    }

    pub fn commit(&mut self, index: RobIndex) {
        debug_assert!(self.committed.is_none() && !self.cleared);

        if !self.zero {
            self.committed = Some(index);
        }
    }
}

impl Sequential for RegStatEntry {
    fn finish_cycle(&mut self) {
        let value = match (self.issued, self.committed) {
            _ if self.cleared => Default::default(),
            (Some(_), _) => self.issued,
            (None, committed) if self.value == committed => None,
            _ => self.value,
        };
        *self = Self {
            value,
            zero: self.zero,
            ..Default::default()
        }
    }
}

impl Clearable for RegStatEntry {
    fn clear(&mut self) {
        self.cleared = true;
    }
}

//#endregion
