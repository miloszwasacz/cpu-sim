use super::{to_index, ZeroBubblePredictor};
use crate::components::cpu::front_end::BranchPredictor;
use crate::components::memory::Address;

use std::collections::HashMap;
use std::mem;

//#region ZbpUpdateLock

#[derive(Debug, Clone, Copy)]
struct ZbpUpdate {
    pc: Address,
    target: Address,
    correct: bool,
    priority: usize,
}

pub struct ZbpUpdateLock<'a> {
    zbp: &'a mut ZeroBubblePredictor,
    updates: Vec<ZbpUpdate>,
}

impl<'a> ZbpUpdateLock<'a> {
    pub(super) fn new(zbp: &'a mut ZeroBubblePredictor) -> Self {
        Self {
            zbp,
            updates: Vec::new(),
        }
    }

    pub fn update(&mut self, pc: Address, target: Address, correct: bool, priority: usize) {
        self.updates.push(ZbpUpdate {
            pc,
            target,
            correct,
            priority,
        });
    }
}

impl Drop for ZbpUpdateLock<'_> {
    fn drop(&mut self) {
        if self.updates.is_empty() {
            return;
        }

        self.updates.sort_unstable_by_key(|update| update.priority);
        let mut entries = HashMap::new();
        for update in mem::take(&mut self.updates) {
            let ZbpUpdate {
                pc,
                target,
                correct,
                priority: _,
            } = update;

            let index = to_index(pc, self.zbp.entries.len());
            let entry = *entries
                .get(&index)
                .unwrap_or_else(|| self.zbp.entries[index].read());
            let entry = match entry {
                entry @ Some((saved_pc, _)) if saved_pc == pc => entry.filter(|_| correct),
                _ if correct => Some((pc, target)),
                entry => entry,
            };
            entries.insert(index, entry);

            if correct {
                self.zbp.correct += 1;
            } else {
                self.zbp.incorrect += 1;
            }
        }

        for (index, entry) in entries {
            self.zbp.entries[index].write(entry);
        }
    }
}

//#endregion

//#region BpUpdateLock

#[derive(Debug, Clone, Copy)]
struct BpUpdate {
    pc: Address,
    taken: bool,
    correct: bool,
    priority: usize,
}

pub struct BpUpdateLock<'a> {
    bp: &'a mut BranchPredictor,
    updates: Vec<BpUpdate>,
}

impl<'a> BpUpdateLock<'a> {
    pub(super) fn new(bp: &'a mut BranchPredictor) -> Self {
        Self {
            bp,
            updates: Vec::new(),
        }
    }

    pub fn update(&mut self, pc: Address, taken: bool, correct: bool, priority: usize) {
        self.updates.push(BpUpdate {
            pc,
            taken,
            correct,
            priority,
        });
    }
}

impl Drop for BpUpdateLock<'_> {
    fn drop(&mut self) {
        if self.updates.is_empty() {
            return;
        }

        self.updates.sort_unstable_by_key(|update| update.priority);
        let mut entries = HashMap::new();
        for update in mem::take(&mut self.updates) {
            let BpUpdate {
                pc,
                taken,
                correct,
                priority: _,
            } = update;

            let index = to_index(pc, self.bp.entries.len());
            let value = *entries
                .get(&index)
                .unwrap_or_else(|| self.bp.entries[index].read());

            entries.insert(index, value.updated(taken));
            if correct {
                self.bp.correct += 1;
            } else {
                self.bp.incorrect += 1;
            }
        }

        for (index, value) in entries {
            self.bp.entries[index].write(value);
        }
    }
}

//#endregion
