use cpu_sim::components::diagnostics::cpu::{RobEntrySnapshot, RobIndex, RobSnapshot};

pub struct RobModel(RobSnapshot);

impl RobModel {
    pub(super) fn new(rob: RobSnapshot) -> Self {
        Self(rob)
    }

    pub fn entries(&self) -> &[(RobIndex, RobEntrySnapshot)] {
        &self.0.entries
    }
}
