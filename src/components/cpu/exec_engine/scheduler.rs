pub(super) use self::entry::{NotReady, Ready, RsEntry};
pub(super) use self::lock::RsLock;
use super::exec_unit::ExecResult;
use super::{OperationType, ReorderBuffer};
use crate::components::cpu::flip_flop::{Clearable, FlipFlop, Sequential};

use std::num::NonZeroUsize;
use std::slice::{Iter, IterMut};

pub mod diagnostics;
mod entry;
mod lock;

//#region ReservationStation

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum ReservationStation {
    #[default]
    Empty,
    NotReady(RsEntry<NotReady>),
    Ready(RsEntry<Ready>),
}

impl ReservationStation {
    pub fn is_empty(&self) -> bool {
        matches!(self, ReservationStation::Empty)
    }
}

impl FlipFlop<ReservationStation> {
    fn take_if_ready(&mut self, rob: &ReorderBuffer) -> Option<RsEntry<Ready>> {
        match self.read() {
            ReservationStation::Empty | ReservationStation::NotReady(_) => None,
            ReservationStation::Ready(entry) => match entry.data {
                Ready::Load1 { .. } if rob.preceding_stores(entry.dest).next().is_some() => {
                    // First step of Load has to wait until there are no outstanding Stores
                    None
                }
                _ => {
                    let entry = *entry;
                    self.write(ReservationStation::Empty);
                    Some(entry)
                }
            },
        }
    }
}

//#endregion

//#region Scheduler

#[derive(Debug)]
pub struct Scheduler {
    ops: OperationType,
    entries: Box<[FlipFlop<ReservationStation>]>,
}

impl Scheduler {
    pub(in crate::components::cpu) fn new(
        ops: impl IntoIterator<Item = OperationType>,
        capacity: NonZeroUsize,
    ) -> Self {
        let ops = OperationType::from_iter(ops);
        let entries =
            vec![FlipFlop::new(ReservationStation::Empty); capacity.get()].into_boxed_slice();
        Self { ops, entries }
    }

    #[cfg(debug_assertions)]
    pub(in crate::components::cpu) fn supported_ops(&self) -> OperationType {
        self.ops
    }

    /// Finds an index of a reservation station.
    /// Returns [`None`] if all reservation stations are busy.
    fn find_empty(&self) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.read().is_empty())
            .map(|(i, _)| i)
            .next()
    }

    /// Takes out the first ready instruction out of the scheduler.
    pub(super) fn take_first_ready(&mut self, rob: &ReorderBuffer) -> Option<RsEntry<Ready>> {
        self.entries.iter_mut().find_map(|rs| rs.take_if_ready(rob))
    }
}

impl Sequential for Scheduler {
    fn finish_cycle(&mut self) {
        for rs in &mut self.entries {
            rs.finish_cycle();
        }
    }
}

impl Clearable for Scheduler {
    fn clear(&mut self) {
        for rs in &mut self.entries {
            rs.clear();
        }
    }
}

//#endregion

//#region Schedulers

#[derive(Debug)]
pub struct Schedulers(Box<[Scheduler]>);

impl Schedulers {
    pub(super) fn reserve(&mut self, op: OperationType) -> Option<RsLock> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, s)| s.ops.contains(op))
            .find_map(|(si, s)| s.find_empty().map(|rs| (si, rs)))
            .map(|(si, rs)| RsLock::new(self, si, rs))
    }

    pub(super) fn iter(&self) -> Iter<Scheduler> {
        self.0.iter()
    }

    pub(super) fn iter_mut(&mut self) -> IterMut<Scheduler> {
        self.0.iter_mut()
    }

    pub(super) fn update_from_cdb(&mut self, results: &[ExecResult]) {
        if results.is_empty() {
            return;
        }

        self.0
            .iter_mut()
            .flat_map(|s| s.entries.iter_mut())
            .for_each(|rs| {
                let current = *rs.read();
                if let ReservationStation::NotReady(entry) = current {
                    let new = match entry.update(results) {
                        Ok(ready) => ReservationStation::Ready(ready),
                        Err(not_ready) => ReservationStation::NotReady(not_ready),
                    };
                    if new != current {
                        rs.write(new);
                    }
                }
            })
    }
}

impl Sequential for Schedulers {
    fn finish_cycle(&mut self) {
        for s in &mut self.0 {
            s.finish_cycle();
        }
    }
}

impl Clearable for Schedulers {
    fn clear(&mut self) {
        for s in &mut self.0 {
            s.clear();
        }
    }
}

impl FromIterator<Scheduler> for Schedulers {
    fn from_iter<T: IntoIterator<Item = Scheduler>>(iter: T) -> Self {
        let mut v = iter.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|s| s.ops.bits());
        Self(v.into_boxed_slice())
    }
}

//#endregion
