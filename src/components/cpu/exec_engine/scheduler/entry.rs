use crate::components::cpu::exec_engine::exec_unit::{ExecResult, ExecResultData};
use crate::components::cpu::exec_engine::rob::{ReadyRobEntry, RobIndex};
use crate::components::cpu::exec_engine::{CommonDataBus, OperationType, ReorderBuffer};
use crate::components::cpu::reg::{FutureFileIssueLock as FutureFile, RegData, RegName};
use crate::components::cpu::{AluControl, MulControl, Pc};
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
            NotReady::Alu { src1, src2, .. }
            | NotReady::Mul { src1, src2, .. }
            | NotReady::Branch { src1, src2, .. } => {
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
            Ok(ExecResultData::Alu(value))
            | Ok(ExecResultData::JumpLink(value))
            | Ok(ExecResultData::Load2(value)) => Some((r.tag, value)),
            _ => None,
        });
        //TODO If entry A is waiting for result B, but result B produces an exception,
        //     entry A will never be free

        for (tag, value) in results {
            match &mut self.data {
                NotReady::Alu { src1, src2, .. }
                | NotReady::Mul { src1, src2, .. }
                | NotReady::Branch { src1, src2, .. } => {
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
            NotReady::Mul {
                ctrl,
                src1: RegValue::Value(src1),
                src2: RegValue::Value(src2),
            } => Ready::Mul { ctrl, src1, src2 },
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
#[derive(Debug, Clone, Copy)]
pub enum RsEntryData<V: RsEntrySrc> {
    Alu {
        /// The control signal specifying which ALU operation should be performed.
        ctrl: AluControl,
        src1: V,
        src2: V,
    },
    Mul {
        /// The control signal specifying which MUL operation should be performed.
        ctrl: MulControl,
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

impl<V: RsEntrySrc + PartialEq> PartialEq for RsEntryData<V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                RsEntryData::Alu {
                    ctrl: c1,
                    src1: s11,
                    src2: s21,
                },
                RsEntryData::Alu {
                    ctrl: c2,
                    src1: s12,
                    src2: s22,
                },
            ) => c1 == c2 && s11 == s12 && s21 == s22,
            (
                RsEntryData::Mul {
                    ctrl: c1,
                    src1: s11,
                    src2: s21,
                },
                RsEntryData::Mul {
                    ctrl: c2,
                    src1: s12,
                    src2: s22,
                },
            ) => c1 == c2 && s11 == s12 && s21 == s22,
            (
                RsEntryData::Jump {
                    base: b1,
                    offset: o1,
                    apply_mask: m1,
                },
                RsEntryData::Jump {
                    base: b2,
                    offset: o2,
                    apply_mask: m2,
                },
            ) => b1 == b2 && o1 == o2 && m1 == m2,
            (
                RsEntryData::Branch {
                    ctrl: c1,
                    src1: s11,
                    src2: s21,
                },
                RsEntryData::Branch {
                    ctrl: c2,
                    src1: s12,
                    src2: s22,
                },
            ) => c1 == c2 && s11 == s12 && s21 == s22,
            (
                RsEntryData::Load1 {
                    load: l1,
                    byte_count: c1,
                    base: b1,
                    offset: o1,
                },
                RsEntryData::Load1 {
                    load: l2,
                    byte_count: c2,
                    base: b2,
                    offset: o2,
                },
            ) => std::ptr::fn_addr_eq(*l1, *l2) && c1 == c2 && b1 == b2 && o1 == o2,
            (
                RsEntryData::Store {
                    base: b1,
                    offset: o1,
                },
                RsEntryData::Store {
                    base: b2,
                    offset: o2,
                },
            ) => b1 == b2 && o1 == o2,
            _ => false,
        }
    }
}

impl NotReady {
    pub(in crate::components::cpu::exec_engine) fn alu(
        ctrl: AluControl,
        src1: Result<RegName, Pc>,
        src2: AluSrcB,
        future_file: &FutureFile,
    ) -> Self {
        let src1 = match src1 {
            Ok(src1) => RegValue::new(src1, future_file),
            Err(src1) => RegValue::Value(RegData::address(src1)),
        };
        let src2 = match src2 {
            AluSrcB::Reg(src2) => RegValue::new(src2, future_file),
            AluSrcB::Imm(src2) => RegValue::Value(RegData::signed(src2)),
        };
        Self::Alu { ctrl, src1, src2 }
    }

    pub(in crate::components::cpu::exec_engine) fn mul(
        ctrl: MulControl,
        src1: RegName,
        src2: RegName,
        future_file: &FutureFile,
    ) -> Self {
        let src1 = RegValue::new(src1, future_file);
        let src2 = RegValue::new(src2, future_file);
        Self::Mul { ctrl, src1, src2 }
    }

    // PC-based jumps don't need RS
    pub(in crate::components::cpu::exec_engine) fn jump(
        base: RegName,
        offset: Immediate,
        apply_mask: bool,
        future_file: &FutureFile,
    ) -> Self {
        let base = RegValue::new(base, future_file);
        Self::Jump {
            base,
            offset,
            apply_mask,
        }
    }

    pub(in crate::components::cpu::exec_engine) fn branch(
        ctrl: Branch,
        src1: RegName,
        src2: RegName,
        future_file: &FutureFile,
    ) -> Self {
        let src1 = RegValue::new(src1, future_file);
        let src2 = RegValue::new(src2, future_file);
        Self::Branch { ctrl, src1, src2 }
    }

    pub(in crate::components::cpu::exec_engine) fn load(
        load: MemRead,
        byte_count: usize,
        base: RegName,
        offset: Immediate,
        future_file: &FutureFile,
    ) -> Self {
        let base = RegValue::new(base, future_file);
        Self::Load1 {
            load,
            byte_count,
            base,
            offset,
        }
    }

    pub(in crate::components::cpu::exec_engine) fn store(
        base: RegName,
        offset: Immediate,
        future_file: &FutureFile,
    ) -> Self {
        let base = RegValue::new(base, future_file);
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
    fn new(reg: RegName, future_file: &FutureFile) -> Self {
        unsafe { future_file.read(reg) }.map_or_else(Self::Rob, Self::Value)
    }

    fn update_from_rob(&mut self, rob: &ReorderBuffer) {
        if let RegValue::Rob(index) = self {
            //TODO If entry A is waiting for result B, but result B produces an exception,
            //     entry A will never be free
            let entry = rob.get_if_ready(*index).and_then(|entry| entry.data().ok());
            if let Some(entry) = entry {
                match entry {
                    ReadyRobEntry::Alu { value, .. } | ReadyRobEntry::Load { value, .. } => {
                        *self = RegValue::Value(value);
                    }
                    ReadyRobEntry::Jump { .. }
                    | ReadyRobEntry::Branch { .. }
                    | ReadyRobEntry::Store { .. }
                    | ReadyRobEntry::Fence
                    | ReadyRobEntry::Csr { .. } => {}
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
