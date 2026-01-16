pub use self::lock::{BpUpdateLock, ZbpUpdateLock};
use self::two_bit::TwoBitPredict;
use crate::components::cpu::error::Exception;
use crate::components::cpu::flip_flop::{FlipFlop, Sequential};
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::raw::{RawInstr, RawInstrType};

use std::num::NonZeroUsize;

pub mod diagnostics;
mod lock;
mod two_bit;

type ZbEntry = Option<(Address, Address)>;

pub struct ZeroBubblePredictor {
    entries: Box<[FlipFlop<ZbEntry>]>,
    correct: usize,
    incorrect: usize,
}

impl ZeroBubblePredictor {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self {
            entries: vec![FlipFlop::new(None); capacity.get()].into_boxed_slice(),
            correct: 0,
            incorrect: 0,
        }
    }

    pub fn predict(&self, pc: Pc) -> Option<Pc> {
        let index = to_index(pc, self.entries.len());
        self.entries[index]
            .read()
            .as_ref()
            .filter(|(addr, _)| *addr == pc)
            .map(|(_, predicted)| *predicted)
    }

    pub fn update_lock(&mut self) -> ZbpUpdateLock<'_> {
        ZbpUpdateLock::new(self)
    }
}

impl Sequential for ZeroBubblePredictor {
    fn finish_cycle(&mut self) {
        self.entries.iter_mut().for_each(FlipFlop::finish_cycle);
    }
}

pub struct BranchPredictor {
    entries: Box<[FlipFlop<TwoBitPredict>]>,
    correct: usize,
    incorrect: usize,
}

impl BranchPredictor {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self {
            entries: vec![FlipFlop::new(Default::default()); capacity.get()].into_boxed_slice(),
            correct: 0,
            incorrect: 0,
        }
    }

    pub fn predict(&self, instr: Result<RawInstr, Exception>, addr: Pc) -> bool {
        let instr = match instr {
            Ok(instr) => instr,
            Err(_) => return false,
        };

        match instr.type_from_opcode() {
            RawInstrType::Regular => false,
            RawInstrType::Jump => true,
            RawInstrType::Branch => {
                let index = to_index(addr, self.entries.len());
                self.entries[index].read().predict()
            }
        }
    }

    pub fn update_lock(&mut self) -> BpUpdateLock<'_> {
        BpUpdateLock::new(self)
    }
}

impl Sequential for BranchPredictor {
    fn finish_cycle(&mut self) {
        self.entries.iter_mut().for_each(FlipFlop::finish_cycle);
    }
}

fn to_index(addr: Address, entry_count: usize) -> usize {
    let shift = entry_count.ilog2();
    let mask = !((Address::MAX >> shift) << shift);
    (addr & mask) as usize
}
