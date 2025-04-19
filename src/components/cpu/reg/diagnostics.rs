use super::data::SignedRegData;
use super::reg_file::{RegFile, RegStat};
pub use super::RegName;
use super::ARCH_REG_COUNT;
use crate::components::cpu::exec_engine::RobIndex;
use crate::components::diagnostics::Diagnostics;

use std::fmt;

pub type RegData = SignedRegData;

pub struct RegFileSnapshot(pub [(RegName, SignedRegData); ARCH_REG_COUNT]);

pub struct RegStatSnapshot(pub [(RegName, RegStatus); ARCH_REG_COUNT]);

pub struct RegStatus(pub(super) Option<RobIndex>);

impl fmt::Display for RegStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(r) => {
                fmt::Display::fmt("#", f)?;
                fmt::Display::fmt(&r, f)
            }
            None => fmt::Display::fmt("", f),
        }
    }
}

impl Diagnostics for RegFile {
    type Output = RegFileSnapshot;

    fn diagnostics(&self) -> Self::Output {
        self.into()
    }
}

impl Diagnostics for RegStat {
    type Output = RegStatSnapshot;

    fn diagnostics(&self) -> Self::Output {
        self.into()
    }
}
