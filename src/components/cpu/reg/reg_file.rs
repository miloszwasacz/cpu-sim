use super::data::RegData;
use super::diagnostics::{FutureFileSnapshot, RegStatSnapshot};
use super::name::RegName;
use super::{Register, ARCH_REG_COUNT};
use crate::components::cpu::flip_flop::Sequential;
use crate::components::cpu::RobIndex;

use std::fmt;
use std::ops::{Index, IndexMut};

//#region RegFile

#[derive(Debug, Clone, Default)]
pub struct RegFile([Register; ARCH_REG_COUNT], FutureFile);

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

    pub fn future_file(&self) -> &FutureFile {
        &self.1
    }

    pub fn future_file_mut(&mut self) -> &mut FutureFile {
        &mut self.1
    }
}

impl Sequential for RegFile {
    fn finish_cycle(&mut self) {
        for reg in &mut self.0 {
            reg.finish_cycle();
        }
        self.1.finish_cycle(&self.0);
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

impl From<&RegFile> for super::diagnostics::RegFileSnapshot {
    fn from(value: &RegFile) -> Self {
        Self(std::array::from_fn(|i| {
            let name = i.try_into().unwrap();
            let data = value.0[i].get().i();
            (name, data)
        }))
    }
}

//#endregion

//#region FutureFile

#[derive(Debug, Clone)]
pub struct FutureFile([RegStat; ARCH_REG_COUNT]);

impl FutureFile {
    pub(in crate::components::cpu) fn issue_lock(&mut self) -> FutureFileIssueLock {
        FutureFileIssueLock {
            file: self,
            issued: [None; ARCH_REG_COUNT],
        }
    }
}

impl Default for FutureFile {
    fn default() -> Self {
        let mut future = Self(Default::default());
        future[RegName::ZERO] = RegStat::zero();
        future
    }
}

impl Index<RegName> for FutureFile {
    type Output = RegStat;

    fn index(&self, index: RegName) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<RegName> for FutureFile {
    fn index_mut(&mut self, index: RegName) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl FutureFile {
    fn finish_cycle(&mut self, reg_file: &[Register; ARCH_REG_COUNT]) {
        for (stat, reg) in self.0.iter_mut().zip(reg_file) {
            stat.finish_cycle(reg);
        }
    }

    pub fn clear(&mut self) {
        for reg in &mut self.0 {
            reg.clear();
        }
    }
}

impl From<&FutureFile> for FutureFileSnapshot {
    fn from(value: &FutureFile) -> Self {
        Self(std::array::from_fn(|i| {
            let name = i.try_into().unwrap();
            let stat = RegStatSnapshot::from(&value.0[i]);
            (name, stat)
        }))
    }
}

//#endregion

//#region FutureFileIssueLock

pub(in crate::components::cpu) struct FutureFileIssueLock<'a> {
    file: &'a mut FutureFile,
    issued: [Option<(usize, RobIndex)>; ARCH_REG_COUNT],
}

//TODO Add a note mentioning that the destructor HAS TO RUN to issue the new instructions
impl FutureFileIssueLock<'_> {
    pub fn read(&self, reg: RegName) -> Result<RegData, RobIndex> {
        self.issued[reg.0]
            .map(|(_, index)| index)
            .map(Err)
            .unwrap_or_else(|| self.file[reg].read())
    }

    pub fn issue_new(&mut self, reg: RegName, index: RobIndex, priority: usize) {
        let stat = &self.file[reg];
        debug_assert!(!stat.cleared);

        if !stat.zero {
            self.issued[reg.0] = match self.issued[reg.0] {
                Some((p, _)) if p < priority => Some((priority, index)),
                None => Some((priority, index)),
                issued => issued,
            }
        }
    }
}

impl Drop for FutureFileIssueLock<'_> {
    fn drop(&mut self) {
        self.file
            .0
            .iter_mut()
            .zip(self.issued.iter())
            .filter_map(|(stat, new)| new.map(|(_, new)| (stat, new)))
            .for_each(|(stat, new)| stat.issue_new(new));
    }
}

//#endregion

//#region RegStat

#[derive(Debug, Clone, Copy, Default)]
pub struct RegStat {
    data: RegData,
    writing: Option<RobIndex>,
    issued: Option<RobIndex>,
    written: Option<RegData>,
    cleared: bool,
    /// Whether this entry is hardwired to the ZERO register
    zero: bool,
}

impl RegStat {
    fn zero() -> Self {
        Self {
            zero: true,
            ..Default::default()
        }
    }

    pub fn read(&self) -> Result<RegData, RobIndex> {
        if self.zero {
            debug_assert!(self.writing.is_none());
            return Ok(RegData::default());
        }

        self.writing.map(Err).unwrap_or(Ok(self.data))
    }

    fn issue_new(&mut self, index: RobIndex) {
        debug_assert!(self.issued.is_none() && !self.cleared);

        if !self.zero {
            self.issued = Some(index);
        }
    }

    pub fn write_result(&mut self, tag: RobIndex, data: RegData) {
        if !matches!(self.writing, Some(index) if index == tag) {
            return;
        }

        debug_assert!(self.written.is_none() && !self.cleared);
        if !self.zero {
            self.written = Some(data);
        }
    }

    fn finish_cycle(&mut self, reg: &Register) {
        let data = if self.cleared {
            reg.get()
        } else {
            self.written.unwrap_or(self.data)
        };
        let writing = match self.issued {
            _ if self.cleared => Default::default(),
            Some(_) => self.issued,
            None if self.written.is_some() => None,
            _ => self.writing,
        };
        *self = Self {
            data,
            writing,
            zero: self.zero,
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        self.cleared = true;
    }
}

impl From<&RegStat> for RegStatSnapshot {
    fn from(value: &RegStat) -> Self {
        Self {
            data: value.data.i(),
            writing: value.writing,
        }
    }
}

//#endregion
