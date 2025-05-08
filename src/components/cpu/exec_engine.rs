pub use self::cdb::CommonDataBus;
pub use self::exec_unit::ExecUnit;
pub use self::rob::{ReorderBuffer, RobIndex, StoreMeta};
pub use self::scheduler::{Scheduler, Schedulers};

use self::exec_unit::ExecResultData;
use self::rob::{ReadyRobEntry, RobEntry};
use self::scheduler::{self as rs, RsEntry};
use super::front_end::decode_queue::Decoded;
use super::mem_subsystem::LoadQueueEntry;
use super::reg::RegData;
use super::Cpu;
use crate::components::memory::Address;
use crate::instr::{AluSrcA, Instruction};

use bitflags::bitflags;
use itertools::{Either, Itertools};

mod cdb;
pub mod diagnostics;
pub mod exec_unit;
mod rob;
mod scheduler;

//#region OperationType

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(super) struct OperationType: u8 {
        const ALU = 1;
        const BRANCH = 1 << 1;
        const STORE = 1 << 2;
        const LOAD = 1 << 3;
    }
}

impl Instruction {
    pub(super) const fn ty(&self) -> OperationType {
        match self {
            Instruction::Alu { .. } | Instruction::Jump { .. } | Instruction::EnvTrap(_) => {
                OperationType::ALU
            }
            Instruction::Branch { .. } => OperationType::BRANCH,
            Instruction::Load { .. } => OperationType::LOAD,
            Instruction::Store { .. } => OperationType::STORE,
        }
    }
}

//#endregion

impl<I, O, E> Cpu<I, O, E> {
    pub(super) fn issue(&mut self) {
        let mut decoded = self.decode_queue.pop();
        let mut rob_lock = self.rob.lock();
        let mut future_file_lock = self.regs.future_file_mut().issue_lock();

        for priority in 0..self.issue_width.get() {
            let rob_entry_lock = match rob_lock.reserve() {
                Some(lock) => lock,
                None => break,
            };
            let decoded_lock = match decoded.head() {
                Some(Ok(decoded)) => decoded,
                Some(Err((ex, pc))) => {
                    let rob_entry = RobEntry::<rob::Ready>::env_trap(pc, ex.into());
                    rob_entry_lock.issue_ready(rob_entry);
                    break;
                }
                None => break,
            };
            let rs_lock = match self.schedulers.reserve(decoded_lock.ty()) {
                Some(rs_lock) => rs_lock,
                _ => break,
            };

            let Decoded {
                instr,
                pc,
                pc_plus_4,
                predicted,
                target,
                ..
            } = decoded_lock.issue();
            let pc_plus_4 = RegData::address(pc_plus_4);

            let rob_index = rob_entry_lock.index();
            let (rob_entry, rs_entry_data, dest) = match instr {
                Instruction::Alu {
                    ctrl,
                    src1,
                    src2,
                    dest,
                } => {
                    let src1 = match src1 {
                        AluSrcA::Reg(src1) => Ok(src1),
                        AluSrcA::Pc => Err(pc),
                    };

                    let rob_entry = RobEntry::alu(pc, dest);
                    let rs_entry_data = rs::NotReady::alu(ctrl, src1, src2, &future_file_lock);
                    (rob_entry, rs_entry_data, Some(dest))
                }
                Instruction::Jump {
                    base: AluSrcA::Reg(base),
                    offset,
                    apply_mask,
                    link_reg,
                } => {
                    let rob_entry =
                        RobEntry::<rob::NotReady>::jump(pc, predicted, link_reg, pc_plus_4);
                    let rs_entry_data =
                        rs::NotReady::jump(base, offset, apply_mask, &future_file_lock);
                    (rob_entry, rs_entry_data, Some(link_reg))
                }
                Instruction::Jump { link_reg, .. } => {
                    // PC-based jumps require no execution, and therefore no reservation stations
                    #[allow(clippy::drop_non_drop)]
                    drop(rs_lock);

                    debug_assert_eq!(predicted, target);
                    let rob_entry =
                        RobEntry::<rob::Ready>::jump(pc, predicted, link_reg, pc_plus_4, target);

                    rob_entry_lock.issue_ready(rob_entry);
                    future_file_lock.issue_new(link_reg, rob_index, priority);
                    self.cdb.write_jump_link(rob_index, pc_plus_4);
                    continue;
                }
                Instruction::Branch {
                    ctrl,
                    src1,
                    src2,
                    offset: _,
                } => {
                    let rob_entry =
                        RobEntry::branch(pc, pc_plus_4.addr(), predicted == target, target);
                    let rs_entry_data = rs::NotReady::branch(ctrl, src1, src2, &future_file_lock);
                    (rob_entry, rs_entry_data, None)
                }
                Instruction::Load {
                    load,
                    byte_count,
                    base,
                    offset,
                    dest,
                } => {
                    let rob_entry = RobEntry::load(pc, dest);
                    let rs_entry_data =
                        rs::NotReady::load(load, byte_count, base, offset, &future_file_lock);
                    (rob_entry, rs_entry_data, Some(dest))
                }
                Instruction::Store {
                    store,
                    byte_count,
                    src,
                    base,
                    offset,
                } => {
                    let rob_entry = RobEntry::store(pc, store, byte_count, src);
                    let rs_entry_data = rs::NotReady::store(base, offset, &future_file_lock);
                    (rob_entry, rs_entry_data, None)
                }
                Instruction::EnvTrap(trap) => {
                    // Trap instructions require no execution, and therefore no reservation stations
                    #[allow(clippy::drop_non_drop)]
                    drop(rs_lock);

                    let rob_entry = RobEntry::<rob::Ready>::env_trap(pc, trap);
                    rob_entry_lock.issue_ready(rob_entry);
                    break;
                }
            };
            let rs_entry = RsEntry {
                dest: rob_index,
                data: rs_entry_data,
                op_type: instr.ty(),
            };

            rob_entry_lock.issue_not_ready(rob_entry);
            match rs_entry.bypass(&rob_lock, &self.cdb) {
                Ok(ready) => rs_lock.issue_ready(ready),
                Err(not_ready) => rs_lock.issue_not_ready(not_ready),
            }
            if let Some(dest) = dest {
                future_file_lock.issue_new(dest, rob_index, priority);
            }
        }
    }

    pub(super) fn execute(&mut self) {
        let rob = &self.rob;
        let schedulers = self.schedulers.iter_mut();
        let exec_units = self.exec_units.iter_mut();
        let load_queue = &mut self.load_queue;
        let mem = self.mem_hierarchy.l1d();

        let mut available_loads = load_queue.free_spaces();
        let (results, load1_results) = schedulers
            .zip(exec_units)
            .filter_map(|(scheduler, exec_unit)| {
                scheduler
                    .take_first_ready(rob)
                    .filter(|entry| {
                        // Loads are restricted by the number of available spaces in the Load Queue
                        !entry.op_type.contains(OperationType::LOAD)
                            || available_loads
                                .checked_sub(1)
                                .inspect(|new| available_loads = *new)
                                .is_some()
                    })
                    .map(|instr| exec_unit.process(instr))
            })
            .chain(load_queue.execute(rob, mem))
            .partition_map(|result| match result.result {
                Ok(ExecResultData::Load1 {
                    load,
                    byte_count,
                    addr,
                }) => Either::Right(LoadQueueEntry::new(result.tag, load, byte_count, addr)),
                _ => Either::Left(result),
            });

        self.cdb.write(results);
        load_queue.write(load1_results);
    }

    pub(super) fn write_result(&mut self) {
        let results = self.cdb.read();
        let future_file = self.regs.future_file_mut();
        self.rob.update_from_cdb(future_file, results);
        self.schedulers.update_from_cdb(results);
    }

    /// Returns then new PC if there was a branch misprediction.
    #[must_use]
    pub(super) fn commit(&mut self) -> Option<Address> {
        let mut rob_lock = self.rob.lock();
        let mut reg_lock = self.regs.commit_lock();
        let mut zbp_lock = self.zb_predictor.update_lock();
        let mut bp_lock = self.branch_predictor.update_lock();
        let mem = self.mem_hierarchy.l1d();

        for priority in 0..self.commit_width.get() {
            let entry = rob_lock.pop_if_ready()?;
            self.stats.executed_instrs += 1;

            let pc = entry.addr();
            let data = match *entry.data() {
                Ok(data) => data,
                Err(trap) => {
                    self.cycle_result.traps.push(trap);
                    return None;
                }
            };

            match data {
                ReadyRobEntry::Alu { dest, value } | ReadyRobEntry::Load { dest, value } => {
                    reg_lock.set(dest, value, priority);
                }
                ReadyRobEntry::Jump {
                    predicted,
                    link_reg,
                    link_data,
                    target,
                } => {
                    reg_lock.set(link_reg, link_data, priority);
                    let correct = predicted == target;
                    zbp_lock.update(pc, target, correct, priority);
                    bp_lock.update(pc, true, correct, priority);
                    self.cycle_result.jump_to_self |= pc == target;
                    if !correct {
                        return Some(target);
                    };
                }
                ReadyRobEntry::Branch {
                    predicted,
                    target,
                    taken,
                    pc_plus_4,
                } => {
                    let correct = predicted == taken;
                    zbp_lock.update(pc, target, correct, priority);
                    bp_lock.update(pc, taken, correct, priority);
                    if !correct {
                        return Some(if taken { target } else { pc_plus_4 });
                    }
                }
                ReadyRobEntry::Store {
                    store,
                    byte_count: _,
                    src,
                    addr,
                } => {
                    // We don't have to model store latency since the result can
                    // be bypassed to any outstanding loads with no delay

                    //TODO Technically, that is not true since loaded and stored data
                    //     might have different addresses but still overlap

                    //TODO Make stores not rely on being commited in the correct order (use priority instead)
                    let value = unsafe { reg_lock.get(src) };
                    store(mem, addr, value);
                }
            }
        }

        None
    }
}
