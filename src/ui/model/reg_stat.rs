use cpu_sim::components::diagnostics::cpu::{RegName, RegStatSnapshot, RegStatus};

pub struct RegStatModel(RegStatSnapshot);

impl RegStatModel {
    pub(super) fn new(reg_stat: RegStatSnapshot) -> Self {
        Self(reg_stat)
    }

    pub fn regs(&self) -> &[(RegName, RegStatus)] {
        &self.0.0
    }
}
