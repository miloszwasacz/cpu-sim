use super::two_bit::TwoBitPredict;
use super::{BranchPredictor, ZeroBubblePredictor};
use crate::components::diagnostics::Diagnostics;
use crate::components::memory::Address;

pub struct ZbPredictorSnapshot {
    entries: Box<[Option<(Address, Address)>]>,
    accuracy: f32,
}

impl ZbPredictorSnapshot {
    #[allow(unused)]
    pub fn entries(&self) -> &[Option<(Address, Address)>] {
        &self.entries
    }

    pub fn accuracy(&self) -> f32 {
        self.accuracy
    }
}

impl Diagnostics for ZeroBubblePredictor {
    type Output = ZbPredictorSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self.entries.iter().map(|f| *f.read()).collect();
        let accuracy = calc_accuracy(self.correct, self.incorrect);
        ZbPredictorSnapshot { entries, accuracy }
    }
}

pub struct BranchPredictorSnapshot {
    entries: Box<[TwoBitPredict]>,
    accuracy: f32,
}

impl BranchPredictorSnapshot {
    #[allow(unused)]
    pub fn entries(&self) -> &[TwoBitPredict] {
        &self.entries
    }

    pub fn accuracy(&self) -> f32 {
        self.accuracy
    }
}

impl Diagnostics for BranchPredictor {
    type Output = BranchPredictorSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self.entries.iter().map(|f| *f.read()).collect();
        let accuracy = calc_accuracy(self.correct, self.incorrect);
        BranchPredictorSnapshot { entries, accuracy }
    }
}

fn calc_accuracy(correct: usize, incorrect: usize) -> f32 {
    let all = correct + incorrect;
    if all > 0 {
        correct as f32 / all as f32
    } else {
        1.0
    }
}
