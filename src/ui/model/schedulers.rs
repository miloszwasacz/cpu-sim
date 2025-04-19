use cpu_sim::components::diagnostics::cpu::SchedulerSnapshot;

pub struct SchedulersModel(Box<[SchedulerSnapshot]>);

impl SchedulersModel {
    pub(super) fn new(schedulers: Box<[SchedulerSnapshot]>) -> Self {
        Self(schedulers)
    }
    
    pub fn schedulers(&self) -> &[SchedulerSnapshot] {
        &self.0
    }
}
