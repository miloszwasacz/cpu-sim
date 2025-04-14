use super::entry::RsData;
pub use super::entry::{NotReady, Ready, RegValue};
use super::{ReservationStation, Scheduler, Schedulers};
use crate::components::cpu::exec_engine::RobIndex;
use crate::components::diagnostics::Diagnostics;

pub struct SchedulerSnapshot {
    pub ops: Box<[&'static str]>,
    pub entries: Box<[SchedulerEntrySnapshot]>,
}

#[allow(private_bounds)]
pub struct SchedulerEntry<D: RsData> {
    pub dest: RobIndex,
    pub data: D,
}

pub enum SchedulerEntrySnapshot {
    Empty,
    NotReady(SchedulerEntry<NotReady>),
    Ready(SchedulerEntry<Ready>),
}

impl From<ReservationStation> for SchedulerEntrySnapshot {
    fn from(value: ReservationStation) -> Self {
        match value {
            ReservationStation::Empty => Self::Empty,
            ReservationStation::NotReady(entry) => Self::NotReady(entry.into()),
            ReservationStation::Ready(entry) => Self::Ready(entry.into()),
        }
    }
}

impl Diagnostics for Scheduler {
    type Output = SchedulerSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let ops = self.ops.iter_names().map(|(name, _)| name).collect();

        let entries = self
            .entries
            .iter()
            .map(|e| e.read())
            .copied()
            .map(SchedulerEntrySnapshot::from)
            .collect();

        Self::Output { ops, entries }
    }
}

impl Diagnostics for Schedulers {
    type Output = Box<[SchedulerSnapshot]>;

    fn diagnostics(&self) -> Self::Output {
        self.0.iter().map(Scheduler::diagnostics).collect()
    }
}
