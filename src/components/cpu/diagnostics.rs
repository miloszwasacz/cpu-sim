pub use super::error::Exception;
pub use super::exec_engine::diagnostics::*;
pub use super::front_end::diagnostics::*;
pub use super::mem_subsystem::diagnostics::*;
pub use super::reg::diagnostics::*;
pub use super::reg::pipeline::{IdIsRegs, IfIdRegs};
pub use super::AluControl;
pub use crate::instr::full::FullInstruction as Instruction;
pub use crate::instr::raw::RawInstr;
pub use crate::instr::Branch;

use super::{Cpu, Pc};
use crate::components::diagnostics::Diagnostics;

pub struct CpuSnapshot {
    pub pc: Pc,
    pub if_id_regs: Box<[Result<RawInstr, Exception>]>,
    pub decode_width: usize,
    pub decode_queue: DecodeQueueSnapshot,
    pub rob: RobSnapshot,
    pub schedulers: Box<[SchedulerSnapshot]>,
    pub reg_file: RegFileSnapshot,
    pub future_file: FutureFileSnapshot,
    pub load_queue: LoadQueueSnapshot,
}

impl<I, O, E> Diagnostics for Cpu<I, O, E> {
    type Output = CpuSnapshot;

    fn diagnostics(&self) -> Self::Output {
        Self::Output {
            pc: *self.pc.read(),
            if_id_regs: self.if_id_regs.read().iter().map(|reg| reg.instr).collect(),
            decode_width: self.decoders.len(),
            decode_queue: self.decode_queue.diagnostics(),
            rob: self.rob.diagnostics(),
            schedulers: self.schedulers.diagnostics(),
            reg_file: self.regs.diagnostics(),
            future_file: self.regs.future_file().diagnostics(),
            load_queue: self.load_queue.diagnostics(&self.rob),
        }
    }
}
