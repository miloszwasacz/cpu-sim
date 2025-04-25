use super::two_bit::TwoBitPredict;
use super::{BranchPredictor, ZeroBubblePredictor};
use crate::components::diagnostics::Diagnostics;
use crate::components::memory::Address;

#[allow(unused)]
pub struct ZbPredictorSnapshot(Box<[Option<(Address, Address)>]>);

impl Diagnostics for ZeroBubblePredictor {
    type Output = ZbPredictorSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self.0.iter().map(|f| *f.read()).collect();
        ZbPredictorSnapshot(entries)
    }
}

#[allow(unused)]
pub struct BranchPredictorSnapshot(Box<[TwoBitPredict]>);

impl Diagnostics for BranchPredictor {
    type Output = BranchPredictorSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let entries = self.0.iter().map(|f| *f.read()).collect();
        BranchPredictorSnapshot(entries)
    }
}
