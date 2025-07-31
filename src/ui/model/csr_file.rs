use cpu_sim::components::diagnostics::cpu::{CsrFileSnapshot, CsrSnapshot};

pub struct CsrFileModel(CsrFileSnapshot);

impl CsrFileModel {
    pub(super) fn new(csr_file: CsrFileSnapshot) -> Self {
        Self(csr_file)
    }

    pub fn csrs(&self) -> &[CsrSnapshot] {
        &self.0.0
    }
}
