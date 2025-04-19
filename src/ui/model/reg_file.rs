use cpu_sim::components::diagnostics::cpu::{RegData, RegFileSnapshot, RegName};

pub struct RegFileModel(RegFileSnapshot);

impl RegFileModel {
    pub(super) fn new(reg_file: RegFileSnapshot) -> Self {
        Self(reg_file)
    }

    pub fn regs(&self) -> &[(RegName, RegData)] {
        &self.0.0
    }
}
