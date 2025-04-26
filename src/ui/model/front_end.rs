use cpu_sim::components::diagnostics::cpu::{
    BpRegs, BranchPredictorSnapshot, CpuStats, IfRegs, ZbPredictorSnapshot, ZbpRegs,
};
use cpu_sim::components::memory::Address;

pub struct FrontEndModel {
    stats: CpuStats,
    pc: Address,
    zb_predictor: ZbPredictorSnapshot,
    zbp_regs: Box<[ZbpRegs]>,
    if_regs: Box<[IfRegs]>,
    branch_predictor: BranchPredictorSnapshot,
    bp_regs: Box<[BpRegs]>,
    decode_width: usize,
}

impl FrontEndModel {
    //TODO Clean up
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        stats: CpuStats,
        pc: Address,
        zb_predictor: ZbPredictorSnapshot,
        zbp_regs: Box<[ZbpRegs]>,
        if_regs: Box<[IfRegs]>,
        branch_predictor: BranchPredictorSnapshot,
        bp_regs: Box<[BpRegs]>,
        decode_width: usize,
    ) -> Self {
        Self {
            stats,
            pc,
            zb_predictor,
            zbp_regs,
            if_regs,
            branch_predictor,
            bp_regs,
            decode_width,
        }
    }

    pub fn stats(&self) -> CpuStats {
        self.stats
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn zb_predictor(&self) -> &ZbPredictorSnapshot {
        &self.zb_predictor
    }

    pub fn zbp_regs(&self) -> &[ZbpRegs] {
        &self.zbp_regs
    }

    pub fn if_regs(&self) -> &[IfRegs] {
        &self.if_regs
    }

    pub fn branch_predictor(&self) -> &BranchPredictorSnapshot {
        &self.branch_predictor
    }

    pub fn bp_regs(&self) -> &[BpRegs] {
        &self.bp_regs
    }

    pub fn decode_width(&self) -> usize {
        self.decode_width
    }
}
