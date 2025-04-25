use self::two_bit::TwoBitPredict;
use crate::components::cpu::error::Exception;
use crate::components::cpu::flip_flop::{FlipFlop, Sequential};
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::raw::{RawInstr, RawInstrType};

use std::num::NonZeroUsize;

pub mod diagnostics;
mod two_bit;

type ZbEntry = Option<(Address, Address)>;

pub struct ZeroBubblePredictor(Box<[FlipFlop<ZbEntry>]>);

impl ZeroBubblePredictor {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self(vec![FlipFlop::new(None); capacity.get()].into_boxed_slice())
    }

    pub fn predict(&self, pc: Pc) -> Option<Pc> {
        let index = to_index(pc, self.0.len());
        self.0[index]
            .read()
            .as_ref()
            .filter(|(addr, _)| *addr == pc)
            .map(|(_, predicted)| *predicted)
    }

    pub fn update(&mut self, pc: Address, target: Address, correct: bool) {
        let index = to_index(pc, self.0.len());
        let entry = match *self.0[index].read() {
            entry @ Some((saved_pc, _)) if saved_pc == pc => entry.filter(|_| correct),
            _ if correct => Some((pc, target)),
            entry => entry,
        };
        self.0[index].write(entry);
    }
}

impl Sequential for ZeroBubblePredictor {
    fn finish_cycle(&mut self) {
        self.0.iter_mut().for_each(FlipFlop::finish_cycle);
    }
}

pub struct BranchPredictor(Box<[FlipFlop<TwoBitPredict>]>);

impl BranchPredictor {
    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self(vec![FlipFlop::new(Default::default()); capacity.get()].into_boxed_slice())
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
                let index = to_index(addr, self.0.len());
                self.0[index].read().predict()
            }
        }
    }

    pub fn update(&mut self, addr: Address, taken: bool) {
        let index = to_index(addr, self.0.len());
        let current = self.0[index].read();
        self.0[index].write(current.updated(taken));
    }
}

impl Sequential for BranchPredictor {
    fn finish_cycle(&mut self) {
        self.0.iter_mut().for_each(FlipFlop::finish_cycle);
    }
}

fn to_index(addr: Address, entry_count: usize) -> usize {
    let shift = entry_count.ilog2();
    let mask = !((Address::MAX >> shift) << shift);
    (addr & mask) as usize
}
