use super::data::SignedRegData;
use super::reg_file::{RegFile, FutureFile};
pub use super::RegName;
use super::ARCH_REG_COUNT;
use crate::components::cpu::exec_engine::RobIndex;
use crate::components::diagnostics::Diagnostics;

pub type RegData = SignedRegData;

pub struct RegFileSnapshot(pub [(RegName, SignedRegData); ARCH_REG_COUNT]);

pub struct FutureFileSnapshot(pub [(RegName, RegStatSnapshot); ARCH_REG_COUNT]);

pub struct RegStatSnapshot {
    pub data: RegData,
    pub writing: Option<RobIndex>,
}

impl Diagnostics for RegFile {
    type Output = RegFileSnapshot;

    fn diagnostics(&self) -> Self::Output {
        self.into()
    }
}

impl Diagnostics for FutureFile {
    type Output = FutureFileSnapshot;

    fn diagnostics(&self) -> Self::Output {
        self.into()
    }
}
