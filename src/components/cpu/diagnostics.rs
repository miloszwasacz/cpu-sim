pub use super::csr::diagnostics::*;
pub use super::error::Exception;
pub use super::exec_engine::diagnostics::*;
pub use super::front_end::diagnostics::*;
pub use super::mem_subsystem::diagnostics::*;
pub use super::reg::diagnostics::*;
pub use super::reg::pipeline::{BpRegs, IfRegs, Prediction, ZbpRegs};
pub use super::AluControl;
pub use crate::instr::full::FullInstruction as Instruction;
pub use crate::instr::raw::RawInstr;
pub use crate::instr::Branch;

use super::{Cpu, Pc};
use crate::components::diagnostics::Diagnostics;

pub struct CpuSnapshot {
    pub stats: CpuStats,
    pub pc: Pc,
    pub zb_predictor: ZbPredictorSnapshot,
    pub zbp_regs: Box<[ZbpRegs]>,
    pub if_regs: Box<[IfRegs]>,
    pub branch_predictor: BranchPredictorSnapshot,
    pub bp_regs: Box<[BpRegs]>,
    pub decode_width: usize,
    pub decode_queue: DecodeQueueSnapshot,
    pub rob: RobSnapshot,
    pub schedulers: Box<[SchedulerSnapshot]>,
    pub reg_file: RegFileSnapshot,
    pub future_file: FutureFileSnapshot,
    pub csr_file: CsrFileSnapshot,
    pub load_queue: LoadQueueSnapshot,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CpuStats {
    pub cycle: u64,
    pub retired: u64,
}

impl CpuStats {
    pub fn ipc(&self) -> f32 {
        if self.cycle == 0 {
            return 0.0;
        }

        self.retired as f32 / self.cycle as f32
    }
}

impl<I, O, E> Diagnostics for Cpu<I, O, E> {
    type Output = CpuSnapshot;

    fn diagnostics(&self) -> Self::Output {
        let stats = CpuStats {
            cycle: self.csr_file.implicit_read(CsrAddr::MCYCLE),
            retired: self.csr_file.implicit_read(CsrAddr::MINSTRET),
        };

        Self::Output {
            stats,
            pc: *self.pc.read(),
            zb_predictor: self.zb_predictor.diagnostics(),
            zbp_regs: self.zbp_regs.read().clone(),
            if_regs: self.if_regs.read().clone(),
            branch_predictor: self.branch_predictor.diagnostics(),
            bp_regs: self.bp_regs.read().clone(),
            decode_width: self.decoders.len(),
            decode_queue: self.decode_queue.diagnostics(),
            rob: self.rob.diagnostics(),
            schedulers: self.schedulers.diagnostics(),
            reg_file: self.regs.diagnostics(),
            future_file: self.regs.future_file().diagnostics(),
            csr_file: self.csr_file.diagnostics(),
            load_queue: self.load_queue.diagnostics(&self.rob),
        }
    }
}
