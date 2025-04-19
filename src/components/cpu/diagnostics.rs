pub use super::error::Exception;
pub use super::exec_engine::diagnostics::*;
pub use super::mem_subsystem::diagnostics::*;
pub use super::reg::diagnostics::*;
pub use super::reg::pipeline::{IdIsRegs, IfIdRegs};
pub use super::AluControl;
pub use crate::instr::full::FullInstruction as Instruction;
pub use crate::instr::Branch;

use super::{Cpu, Pc};
use crate::components::diagnostics::Diagnostics;

pub struct CpuSnapshot {
    pub pc: Pc,
    pub if_id_regs: Option<IfIdRegs>,
    pub id_is_regs: Option<IdIsRegs>,
    pub rob: RobSnapshot,
    pub schedulers: Box<[SchedulerSnapshot]>,
    pub reg_file: RegFileSnapshot,
    pub reg_stat: RegStatSnapshot,
    pub load_queue: LoadQueueSnapshot,
}

impl Diagnostics for Cpu<'_> {
    type Output = CpuSnapshot;

    fn diagnostics(&self) -> Self::Output {
        Self::Output {
            pc: *self.pc.read(),
            if_id_regs: *self.if_id_regs.read(),
            id_is_regs: *self.id_is_regs.read(),
            rob: self.rob.diagnostics(),
            schedulers: self.schedulers.diagnostics(),
            reg_file: self.regs.diagnostics(),
            reg_stat: self.regs.stat().diagnostics(),
            load_queue: self.load_queue.diagnostics(&self.rob),
        }
    }
}
