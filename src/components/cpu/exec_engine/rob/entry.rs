use crate::components::cpu::exec_engine::exec_unit::{ExecResult, ExecResultData};
use crate::components::cpu::reg::{RegData, RegName};
use crate::components::cpu::Pc;
use crate::components::memory::Address;
use crate::instr::mem_access::MemWrite;
use crate::instr::EnvTrap;

pub type NotReady = NotReadyRobEntry;
pub type Ready = Result<ReadyRobEntry, EnvTrap>;

//#region RobEntry

#[allow(private_bounds)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RobEntry<D: RobEntryData>(D, Address);

impl RobEntry<NotReady> {
    pub fn alu(pc: Pc, dest: RegName) -> Self {
        Self(NotReady::Alu { dest }, pc)
    }

    pub fn jump(pc: Pc, predicted: Address, link_reg: RegName, link_data: RegData) -> Self {
        Self(
            NotReady::Jump {
                predicted,
                link_reg,
                link_data,
            },
            pc,
        )
    }

    pub fn branch(pc: Pc, pc_plus_4: Pc, predicted: bool, target: Address) -> Self {
        Self(
            NotReady::Branch {
                predicted,
                target,
                pc_plus_4,
            },
            pc,
        )
    }

    pub fn load(pc: Pc, dest: RegName) -> Self {
        Self(NotReady::Load { dest }, pc)
    }

    pub fn store(pc: Pc, store: MemWrite, byte_count: usize, src: RegName) -> Self {
        Self(
            NotReady::Store {
                store,
                byte_count,
                src,
            },
            pc,
        )
    }

    pub(super) fn data(&self) -> &NotReady {
        &self.0
    }

    pub(super) fn update(self, result: &ExecResult) -> RobEntry<Ready> {
        macro_rules! extract_result {
            ($result:expr, $pat:pat => {
                $( $stmt:tt )*
            }) => {
                match $result {
                    $pat => {
                        $( $stmt )*
                    },
                    _ => unreachable!("result type doesn't match ROB entry type"),
                }
            };
        }

        let data = match result.result {
            Ok(data) => data,
            Err(ex) => return RobEntry(Err(EnvTrap::Exception(ex)), self.1),
        };
        let ready = match self.0 {
            NotReady::Alu { dest } => extract_result!(data, ExecResultData::Alu(value) => {
                ReadyRobEntry::Alu { dest, value }
            }),
            NotReady::Jump {
                predicted,
                link_reg,
                link_data,
            } => extract_result!(data, ExecResultData::Jump { target } => {
                ReadyRobEntry::Jump {
                    predicted,
                    link_reg,
                    link_data,
                    target,
                }
            }),
            NotReady::Branch {
                predicted,
                target,
                pc_plus_4,
            } => {
                extract_result!(data, ExecResultData::Branch { taken } => {
                    ReadyRobEntry::Branch {
                        predicted,
                        target,
                        taken,
                        pc_plus_4,
                    }
                })
            }
            NotReady::Load { dest } => extract_result!(data, ExecResultData::Load2(value) => {
                ReadyRobEntry::Load { dest, value }
            }),
            NotReady::Store {
                store,
                byte_count,
                src,
            } => {
                extract_result!(data, ExecResultData::Store(addr) => {
                    ReadyRobEntry::Store { store, byte_count, src, addr }
                })
            }
        };

        RobEntry(Ok(ready), self.1)
    }
}

impl RobEntry<Ready> {
    pub fn jump(
        pc: Pc,
        predicted: Address,
        link_reg: RegName,
        link_data: RegData,
        target: Address,
    ) -> Self {
        Self(
            Ok(ReadyRobEntry::Jump {
                predicted,
                link_reg,
                link_data,
                target,
            }),
            pc,
        )
    }

    pub fn env_trap(pc: Pc, trap: EnvTrap) -> Self {
        Self(Err(trap), pc)
    }

    pub fn fence(pc: Pc) -> Self {
        Self(Ok(ReadyRobEntry::Fence), pc)
    }

    pub fn data(&self) -> &Ready {
        &self.0
    }

    /// Address of the instruction.
    pub fn addr(&self) -> Address {
        self.1
    }
}

impl<D: RobEntryData> From<RobEntry<D>> for super::diagnostics::RobEntry<D> {
    fn from(value: RobEntry<D>) -> Self {
        Self {
            addr: value.1,
            data: value.0,
        }
    }
}

//#endregion

//#region NotReady

//TODO Improve docs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotReadyRobEntry {
    Alu {
        /// The register where the result will be put.
        dest: RegName,
    },
    Jump {
        //TODO Add link to branch predictor
        /// The predicted target address computed by the Branch Predictor. \
        /// Should be supplied when the instruction is issued.
        predicted: Address,
        /// The link register. \
        /// Should be supplied when the instruction is issued.
        link_reg: RegName,
        /// Address of the instruction following the jump (PC + 4). \
        /// Should be supplied when the instruction is issued.
        link_data: RegData,
    },
    Branch {
        /// The prediction of whether the branch is taken computed by the Branch Predictor. \
        /// Should be supplied when the instruction is issued.
        predicted: bool,
        /// The target address computed assuming the branch is taken, computed during decode.
        /// Should be supplied when the instruction is issued.
        target: Address,
        pc_plus_4: Address,
    },
    Load {
        /// The register where the result will be put.
        dest: RegName,
    },
    Store {
        /// The function to store the data.
        store: MemWrite,
        /// How long is the stored data.
        byte_count: usize,
        /// The register holding the value to be stored
        src: RegName,
    },
}

//#endregion

//#region Ready

//TODO Improve docs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadyRobEntry {
    Alu {
        /// The register where the result will be put.
        dest: RegName,
        /// The result of the operation.
        value: RegData,
    },
    Jump {
        //TODO Add link to branch predictor
        /// The predicted target address computed by the Branch Predictor. \
        /// Should be supplied when the instruction is issued.
        predicted: Address,
        /// The link register
        link_reg: RegName,
        /// Address of the instruction following the jump (PC + 4). \
        /// Should be supplied when the instruction is issued.
        link_data: RegData,
        /// The target address of the jump. \
        /// May be supplied on issue if it can be easily computed during decode (i.e. is based on PC).
        target: Address,
    },
    Branch {
        /// The prediction of whether the branch is taken computed by the Branch Predictor. \
        /// Should be supplied when the instruction is issued.
        predicted: bool,
        /// The target address computed assuming the branch is taken, computed during decode.
        /// Should be supplied when the instruction is issued.
        target: Address,
        /// Whether the branch has actually been taken.
        taken: bool,
        pc_plus_4: Address,
    },
    Load {
        /// The register where the result will be put.
        dest: RegName,
        /// The result of the load.
        value: RegData,
    },
    Store {
        /// The function to store the data.
        store: MemWrite,
        /// How long is the stored data.
        byte_count: usize,
        /// The register holding the value to be stored
        src: RegName, // We hold the register name instead of the value since we can just get it on instruction commit
        /// The effective address.
        addr: Address,
    },
    Fence,
}

//#endregion

pub(super) trait RobEntryData {}
impl RobEntryData for NotReady {}
impl RobEntryData for Ready {}
