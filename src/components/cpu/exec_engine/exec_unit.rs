use self::agu::Agu;
use self::alu::Alu;
use self::branch::BranchUnit;
use super::rob::RobIndex;
use super::scheduler::{Ready, RsEntry, Scheduler};
use super::{OperationType, Schedulers};
use crate::components::cpu::error::Exception;
use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::mem_access::MemRead;

pub mod agu;
pub mod alu;
pub mod branch;

//#region ExecUnit

#[cfg(debug_assertions)]
pub struct ExecUnit {
    ops: OperationType,
}

#[cfg(not(debug_assertions))]
pub struct ExecUnit(());

impl ExecUnit {
    #[inline]
    fn new(scheduler: &Scheduler) -> Self {
        #[cfg(debug_assertions)]
        {
            let ops = scheduler.supported_ops();
            Self { ops }
        }
        #[cfg(not(debug_assertions))]
        {
            Self(())
        }
    }

    pub fn process(&mut self, instr: RsEntry<Ready>) -> ExecResult {
        debug_assert!(
            self.ops.contains(instr.op_type),
            "invalid instruction type: supported {:?}, got {:?}",
            self.ops,
            instr.op_type
        );

        ExecResult {
            tag: instr.dest,
            result: instr.data.try_into(),
        }
    }
}

impl From<&Schedulers> for Vec<ExecUnit> {
    fn from(value: &Schedulers) -> Self {
        value.iter().map(ExecUnit::new).collect()
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

#[derive(Debug, Clone, Copy)]
pub enum ExecResultData {
    Alu(RegData),
    Jump {
        target: Address,
    },
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

impl TryFrom<Ready> for ExecResultData {
    type Error = Exception;

    fn try_from(value: Ready) -> Result<Self, Self::Error> {
        //TODO Exceptions

        Ok(match value {
            Ready::Alu { ctrl, src1, src2 } => {
                let result = Alu.process(ctrl, src1, src2);
                Self::Alu(result)
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
        })
    }
}

//#endregion
