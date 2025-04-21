use super::{NotReady, Ready, ReservationStation, RsEntry, Schedulers};
use crate::components::cpu::flip_flop::FlipFlop;

pub(in crate::components::cpu::exec_engine) struct RsLock<'a> {
    schedulers: &'a mut Schedulers,
    scheduler_index: usize,
    rs_index: usize,
}

impl<'a> RsLock<'a> {
    pub(super) fn new(
        schedulers: &'a mut Schedulers,
        scheduler_index: usize,
        rs_index: usize,
    ) -> Self {
        Self {
            schedulers,
            scheduler_index,
            rs_index,
        }
    }

    pub fn issue_not_ready(self, entry: RsEntry<NotReady>) {
        let rs = self.into_rs();
        rs.write(ReservationStation::NotReady(entry));
    }

    pub fn issue_ready(self, entry: RsEntry<Ready>) {
        let rs = self.into_rs();
        rs.write(ReservationStation::Ready(entry));
    }

    fn into_rs(self) -> &'a mut FlipFlop<ReservationStation> {
        let Self {
            schedulers,
            scheduler_index,
            rs_index,
        } = self;
        let rs = &mut schedulers.schedulers[scheduler_index].entries[rs_index];
        schedulers.reserved.insert(scheduler_index);
        debug_assert!(rs.read().is_empty());
        rs
    }
}
