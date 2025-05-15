use self::agu::Agu;
use self::alu::Alu;
use self::branch::BranchUnit;
use self::mul::Mul;
use super::rob::RobIndex;
use super::scheduler::{Ready, RsEntry, Scheduler};
#[cfg(debug_assertions)]
use super::OperationType;
use super::Schedulers;
use crate::components::cpu::error::Exception;
use crate::components::cpu::flip_flop::{Clearable, FlipFlop, Sequential};
use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::mem_access::MemRead;

use std::num::NonZeroUsize;

pub mod agu;
pub mod alu;
pub mod branch;
pub mod mul;

//#region ExecUnit

#[derive(Debug, Clone, Copy, Default)]
enum Execution {
    #[default]
    Idle,
    Executing {
        result: ExecResult,
        cycles: NonZeroLatency,
    },
}

pub struct ExecUnit {
    executing: FlipFlop<Execution>,
    #[cfg(debug_assertions)]
    ops: OperationType,
}

impl ExecUnit {
    fn new(_scheduler: &Scheduler) -> Self {
        Self {
            executing: FlipFlop::new(Execution::Idle),
            #[cfg(debug_assertions)]
            ops: _scheduler.supported_ops(),
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.executing.read(), Execution::Idle)
    }

    //TODO Add link to `is_idle` in the docs
    pub fn process_new(&mut self, instr: RsEntry<Ready>) -> Option<ExecResult> {
        #[cfg(debug_assertions)]
        {
            assert!(
                self.ops.contains(instr.op_type),
                "invalid instruction type: supported {:?}, got {:?}",
                self.ops,
                instr.op_type
            );
            assert!(
                matches!(self.executing.read(), Execution::Idle),
                "another instruction already executing"
            );
        }

        let (data, cycles) = ExecResultData::from(instr.data);
        let result = ExecResult {
            tag: instr.dest,
            result: Ok(data),
        };
        self.update_executing(result, cycles)
    }

    //TODO Add link to `is_idle` in the docs
    pub fn process_running(&mut self) -> Option<ExecResult> {
        let (result, cycles) = match *self.executing.read() {
            Execution::Executing { result, cycles } => (result, cycles),
            Execution::Idle => panic!("no instruction executing"),
        };
        self.update_executing(result, cycles)
    }

    fn update_executing(
        &mut self,
        result: ExecResult,
        cycles: NonZeroLatency,
    ) -> Option<ExecResult> {
        let new_cycles: Latency = cycles.get() - 1;
        match NonZeroLatency::new(new_cycles) {
            None => {
                self.executing.write(Execution::Idle);
                Some(result)
            }
            Some(cycles) => {
                let execution = Execution::Executing { result, cycles };
                self.executing.write(execution);
                None
            }
        }
    }
}

impl From<&Schedulers> for Vec<ExecUnit> {
    fn from(value: &Schedulers) -> Self {
        value.iter().map(ExecUnit::new).collect()
    }
}

impl Sequential for ExecUnit {
    fn finish_cycle(&mut self) {
        self.executing.finish_cycle();
    }
}

impl Clearable for ExecUnit {
    fn clear(&mut self) {
        self.executing.clear();
    }
}

//#endregion

//#region ExecResult

#[derive(Debug, Clone, Copy)]
pub struct ExecResult {
    pub(super) tag: RobIndex,
    pub(super) result: Result<ExecResultData, Exception>,
}

impl ExecResult {
    pub(in crate::components::cpu) fn load2(
        tag: RobIndex,
        result: Result<RegData, Exception>,
    ) -> Self {
        Self {
            tag,
            result: result.map(ExecResultData::Load2),
        }
    }
}

//#endregion

//#region ExecResultData

type Latency = usize;
type NonZeroLatency = NonZeroUsize;

const DEFAULT_LATENCY: NonZeroLatency = NonZeroLatency::new(1 as Latency).unwrap();

#[derive(Debug, Clone, Copy)]
pub enum ExecResultData {
    Alu(RegData),
    Jump {
        target: Address,
    },
    JumpLink(RegData),
    Branch {
        taken: bool,
    },
    Load1 {
        load: MemRead,
        byte_count: usize,
        addr: Address,
    },
    Load2(RegData),
    Store(Address),
}

impl ExecResultData {
    fn from(value: Ready) -> (Self, NonZeroLatency) {
        let data = match value {
            Ready::Alu { ctrl, src1, src2 } => {
                let result = Alu.process(ctrl, src1, src2);
                Self::Alu(result)
            }
            Ready::Mul { ctrl, src1, src2 } => {
                let (result, latency) = Mul.process(ctrl, src1, src2);
                return (Self::Alu(result), latency);
            }
            Ready::Jump {
                base,
                offset,
                apply_mask,
            } => {
                let result = Alu.jump_target(base, offset, apply_mask);
                Self::Jump { target: result }
            }
            Ready::Branch { ctrl, src1, src2 } => {
                let result = BranchUnit.branch(ctrl, src1, src2);
                Self::Branch { taken: result }
            }
            Ready::Load1 {
                load,
                byte_count,
                base,
                offset,
            } => {
                let result = Agu.addr(base, offset);
                Self::Load1 {
                    load,
                    byte_count,
                    addr: result,
                }
            }
            Ready::Store { base, offset } => {
                let result = Agu.addr(base, offset);
                Self::Store(result)
            }
        };
        (data, DEFAULT_LATENCY)
    }
}

//#endregion
