use cpu_sim::components::diagnostics::cpu::{RegName, FutureFileSnapshot, RegStatSnapshot};

pub struct FutureFileModel(FutureFileSnapshot);

impl FutureFileModel {
    pub(super) fn new(future_file: FutureFileSnapshot) -> Self {
        Self(future_file)
    }

    pub fn regs(&self) -> &[(RegName, RegStatSnapshot)] {
        &self.0.0
    }
}
