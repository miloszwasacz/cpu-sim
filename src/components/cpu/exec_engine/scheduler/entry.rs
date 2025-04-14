use crate::components::cpu::exec_engine::exec_unit::{ExecResult, ExecResultData};
use crate::components::cpu::exec_engine::rob::{ReadyRobEntry, RobIndex};
use crate::components::cpu::exec_engine::{CommonDataBus, OperationType, ReorderBuffer};
use crate::components::cpu::reg::{RegData, RegFile, RegName};
use crate::components::cpu::{AluControl, Pc};
use crate::instr::mem_access::MemRead;
use crate::instr::{AluSrcB, Branch, Immediate};

pub type NotReady = RsEntryData<RegValue>;
pub type Ready = RsEntryData<RegData>;

//#region RsEntry

#[allow(private_bounds)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RsEntry<D: RsData> {
    pub dest: RobIndex,
    pub data: D,
    pub(in crate::components::cpu) op_type: OperationType,
}

impl RsEntry<NotReady> {
    pub(in crate::components::cpu::exec_engine) fn bypass(
        mut self,
        rob: &ReorderBuffer,
        cdb: &CommonDataBus,
    ) -> Result<RsEntry<Ready>, RsEntry<NotReady>> {
        match &mut self.data {
            NotReady::Alu { src1, src2, .. } | NotReady::Branch { src1, src2, .. } => {
                src1.update_from_rob(rob);
                src2.update_from_rob(rob);
            }
            NotReady::Jump { base, .. }
            | NotReady::Load1 { base, .. }
            | NotReady::Store { base, .. } => {
                base.update_from_rob(rob);
            }
        };

        self.update(cdb.read())
    }

    pub(super) fn update(
        mut self,
        results: &[ExecResult],
    ) -> Result<RsEntry<Ready>, RsEntry<NotReady>> {
        let results = results.iter().copied().filter_map(|r| match r.result {
            Ok(ExecResultData::Alu(value)) | Ok(ExecResultData::Load2(value)) => {
                Some((r.tag, value))
            }
            _ => None,
        });
        //TODO If entry A is waiting for result B, but result B produces an exception,
        //     entry A will never be free

        for (tag, value) in results {
            match &mut self.data {
                NotReady::Alu { src1, src2, .. } | NotReady::Branch { src1, src2, .. } => {
                    src1.update_from_cdb(tag, value);
                    src2.update_from_cdb(tag, value);
                }
                NotReady::Jump { base, .. }
                | NotReady::Load1 { base, .. }
                | NotReady::Store { base, .. } => {
                    base.update_from_cdb(tag, value);
                }
            };
        }
        self.try_into()
    }
}

impl TryFrom<RsEntry<NotReady>> for RsEntry<Ready> {
    type Error = RsEntry<NotReady>;

    fn try_from(value: RsEntry<NotReady>) -> Result<Self, Self::Error> {
        let ready = match value.data {
            NotReady::Alu {
                ctrl,
                src1: RegValue::Value(src1),
                src2: RegValue::Value(src2),
            } => Ready::Alu { ctrl, src1, src2 },
            NotReady::Jump {
                base: RegValue::Value(base),
                offset,
                apply_mask,
            } => Ready::Jump {
                base,
                offset,
                apply_mask,
            },
            NotReady::Branch {
                ctrl,
                src1: RegValue::Value(src1),
                src2: RegValue::Value(src2),
            } => Ready::Branch { ctrl, src1, src2 },
            NotReady::Load1 {
                load,
                byte_count,
                base: RegValue::Value(base),
                offset,
            } => Ready::Load1 {
                load,
                byte_count,
                base,
                offset,
            },
            NotReady::Store {
                base: RegValue::Value(base),
                offset,
            } => Ready::Store { base, offset },
            _ => return Err(value),
        };

        Ok(Self {
            dest: value.dest,
            data: ready,
            op_type: value.op_type,
        })
    }
}

impl<D: RsData> From<RsEntry<D>> for super::diagnostics::SchedulerEntry<D> {
    fn from(value: RsEntry<D>) -> Self {
        Self {
            dest: value.dest,
            data: value.data,
        }
    }
}

//#endregion

//#region RsEntryData

#[allow(private_bounds)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RsEntryData<V: RsEntrySrc> {
    Alu {
        /// The control signal specifying which ALU operation should be performed.
        ctrl: AluControl,
        src1: V,
        src2: V,
    },
    Jump {
        base: V, // From Regs or <PC supplied on issue>
        offset: Immediate,
        apply_mask: bool, // Constant, based on the jump type
    },
    Branch {
        ctrl: Branch,
        src1: V,
        src2: V,
    },
    /// First step of LOAD instructions - address generation
    Load1 {
        load: MemRead,
        /// How long is the stored data.
        byte_count: usize,
        base: V,
        offset: Immediate,
    },
    Store {
        base: V,
        offset: Immediate,
    },
}

impl NotReady {
    pub fn alu(ctrl: AluControl, src1: Result<RegName, Pc>, src2: AluSrcB, regs: &RegFile) -> Self {
        let src1 = match src1 {
            Ok(src1) => RegValue::new(src1, regs),
            Err(src1) => RegValue::Value(RegData::address(src1)),
        };
        let src2 = match src2 {
            AluSrcB::Reg(src2) => RegValue::new(src2, regs),
            AluSrcB::Imm(src2) => RegValue::Value(RegData::signed(src2)),
        };
        Self::Alu { ctrl, src1, src2 }
    }

    // PC-based jumps don't need RS
    pub fn jump(base: RegName, offset: Immediate, apply_mask: bool, regs: &RegFile) -> Self {
        let base = RegValue::new(base, regs);
        Self::Jump {
            base,
            offset,
            apply_mask,
        }
    }

    pub fn branch(ctrl: Branch, src1: RegName, src2: RegName, regs: &RegFile) -> Self {
        let src1 = RegValue::new(src1, regs);
        let src2 = RegValue::new(src2, regs);
        Self::Branch { ctrl, src1, src2 }
    }

    pub fn load(
        load: MemRead,
        byte_count: usize,
        base: RegName,
        offset: Immediate,
        regs: &RegFile,
    ) -> Self {
        let base = RegValue::new(base, regs);
        Self::Load1 {
            load,
            byte_count,
            base,
            offset,
        }
    }

    pub fn store(base: RegName, offset: Immediate, regs: &RegFile) -> Self {
        let base = RegValue::new(base, regs);
        Self::Store { base, offset }
    }
}

//#endregion

//#region RegValue

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegValue {
    Rob(RobIndex),
    Value(RegData),
}

impl RegValue {
    fn new(reg: RegName, regs: &RegFile) -> RegValue {
        regs.stat()[reg]
            .read()
            .map(RegValue::Rob)
            .unwrap_or_else(|| {
                let val = regs.get(reg);
                RegValue::Value(val)
            })
    }

    fn update_from_rob(&mut self, rob: &ReorderBuffer) {
        if let RegValue::Rob(index) = self {
            //TODO If entry A is waiting for result B, but result B produces an exception,
            //     entry A will never be free
            let entry = rob.get_if_ready(*index).and_then(|entry| entry.data().ok());
            if let Some(entry) = entry {
                match entry {
                    ReadyRobEntry::Alu { value, .. }
                    | ReadyRobEntry::Load { value, .. }
                    | ReadyRobEntry::Jump {
                        link_data: value, ..
                    } => {
                        *self = RegValue::Value(value);
                    }
                    ReadyRobEntry::Branch { .. } | ReadyRobEntry::Store { .. } => {}
                }
            }
        }
    }

    fn update_from_cdb(&mut self, rob_index: RobIndex, value: RegData) {
        match self {
            RegValue::Rob(index) if *index == rob_index => {
                *self = RegValue::Value(value);
            }
            _ => {}
        }
    }
}

//#endregion

pub(super) trait RsData {}
impl RsData for NotReady {}
impl RsData for Ready {}

trait RsEntrySrc {}
impl RsEntrySrc for RegData {}
impl RsEntrySrc for RegValue {}
