use super::circuit::ClockCycle;
use super::exec_engine::ExecutionEngine;
use super::reg::pipeline::{
    DecodeRegs, ExecuteControl, ExecuteRegs, FetchRegs, IssueControl, IssueRegs, MemAccessRegs,
    PipelineRegs, WritebackControl, WritebackRegs,
};
use super::reg::{RegData, RegName};
use crate::instr::issue::Branch;

pub struct HazardUnit {
    fetch_regs: PipelineRegs<FetchRegs>,
    decode_regs: PipelineRegs<DecodeRegs>,
    issue_regs: PipelineRegs<IssueRegs>,
    execute_regs: PipelineRegs<ExecuteRegs>,
    mem_access_regs: PipelineRegs<MemAccessRegs>,
    writeback_regs: PipelineRegs<WritebackRegs>,
    tick_state: TickState,
}

impl HazardUnit {
    pub fn new(
        fetch_regs: &PipelineRegs<FetchRegs>,
        decode_regs: &PipelineRegs<DecodeRegs>,
        issue_regs: &PipelineRegs<IssueRegs>,
        execute_regs: &PipelineRegs<ExecuteRegs>,
        mem_access_regs: &PipelineRegs<MemAccessRegs>,
        writeback_regs: &PipelineRegs<WritebackRegs>,
    ) -> Self {
        let fetch_regs = fetch_regs.clone();
        let decode_regs = decode_regs.clone();
        let issue_regs = issue_regs.clone();
        let execute_regs = execute_regs.clone();
        let mem_access_regs = mem_access_regs.clone();
        let writeback_regs = writeback_regs.clone();
        let tick_state = Default::default();

        Self {
            fetch_regs,
            decode_regs,
            issue_regs,
            execute_regs,
            mem_access_regs,
            writeback_regs,
            tick_state,
        }
    }

    pub fn start_cycle(&mut self) {
        self.tick_state = TickState {
            fetch_regs: self.fetch_regs.read(ClockCycle::FirstHalf),
            decode_regs: self.decode_regs.read(ClockCycle::FirstHalf),
            issue_regs: self.issue_regs.read(ClockCycle::FirstHalf),
            execute_regs: self.execute_regs.read(ClockCycle::FirstHalf),
            execute_out: None,
            mem_access_regs: self.mem_access_regs.read(ClockCycle::FirstHalf),
            mem_access_out: None,
            writeback_regs: self.writeback_regs.read(ClockCycle::FirstHalf),
        };

        let IssueRegs {
            is_ctrl: IssueControl { branch: branch_i },
            ex_ctrl:
                ExecuteControl {
                    env_trap: env_trap_i,
                    ..
                },
            rs1: rs1_i,
            rs2: rs2_i,
            ..
        } = self.tick_state.issue_regs;
        let ExecuteRegs {
            ex_ctrl: ExecuteControl { jump: jump_e, .. },
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_e,
                    reg_write: reg_write_e,
                },
            write_reg: write_reg_e,
            ..
        } = self.tick_state.execute_regs;
        let MemAccessRegs {
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_m,
                    reg_write: reg_write_m,
                },
            write_reg: write_reg_m,
            ..
        } = self.tick_state.mem_access_regs;
        let branch_i = branch_i != Branch::None;
        let env_trap_i = env_trap_i.is_some();

        let load_stall = {
            // Operand for load in Execute stage
            let stall_e = mem_to_reg_e
                && (reg_eq_non_zero(rs1_i, write_reg_e) || reg_eq_non_zero(rs2_i, write_reg_e))
                && reg_write_e;

            // Operand for load in will be loaded from memory
            let stall_m = mem_to_reg_m
                && (reg_eq_non_zero(rs1_i, write_reg_m) || reg_eq_non_zero(rs2_i, write_reg_m))
                && reg_write_m;

            stall_e || stall_m
        };
        let branch_stall = {
            // Operand for branch in Execute stage will be loaded from memory
            branch_i
                && mem_to_reg_e
                && (reg_eq_non_zero(rs1_i, write_reg_e) || reg_eq_non_zero(rs2_i, write_reg_e))
                && reg_write_e
        };
        let env_stall = {
            // Instruction in Execute stage modifies registers
            let stall_e = reg_write_e && !write_reg_e.is_zero();

            // Instruction in Memory Access stage modifies registers
            let stall_m = reg_write_m && !write_reg_m.is_zero();

            env_trap_i && (stall_e || stall_m)
        };

        let stall = load_stall || branch_stall || env_stall;
        let stall_f = stall;
        let stall_d = stall;
        let stall_i = stall;
        let flush_d = jump_e;
        let flush_e = stall;

        self.fetch_regs.en(!stall_f);
        self.decode_regs.en(!stall_d);
        self.issue_regs.en(!stall_i);
        self.decode_regs.clr(flush_d, ClockCycle::SecondHalf);
        self.execute_regs.clr(flush_e, ClockCycle::SecondHalf);
    }

    pub fn execute_forward_out(&mut self, regs: MemAccessRegs) {
        self.tick_state.execute_out = Some(regs);
    }

    pub fn mem_access_forward_out(&mut self, regs: WritebackRegs) {
        self.tick_state.mem_access_out = Some(regs);
    }

    //noinspection DuplicatedCode
    pub fn issue_forward_in(
        &self,
        (rs1, src1): (RegName, RegData),
        (rs2, src2): (RegName, RegData),
    ) -> (RegData, RegData) {
        let MemAccessRegs {
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_e,
                    reg_write: reg_write_e,
                },
            alu_out: alu_out_e,
            write_reg: write_reg_e,
            ..
        } = self
            .tick_state
            .execute_out
            .expect("the Execute stage should be performed before the Issue stage");
        let WritebackRegs {
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_m,
                    reg_write: reg_write_m,
                },
            alu_out: alu_out_m,
            read_data: read_data_m,
            write_reg: write_reg_m,
            ..
        } = self
            .tick_state
            .mem_access_out
            .expect("the Memory Access stage should be performed before the Issue stage");

        let forward_single = |r: RegName, src: RegData| {
            if reg_eq_non_zero(r, write_reg_e) && reg_write_e && !mem_to_reg_e {
                alu_out_e
            } else if reg_eq_non_zero(r, write_reg_m) && reg_write_m {
                ExecutionEngine::writeback_mutex(mem_to_reg_m, alu_out_m, read_data_m)
            } else {
                src
            }
        };

        let src_a = forward_single(rs1, src1);
        let src_b = forward_single(rs2, src2);
        (src_a, src_b)
    }

    //noinspection DuplicatedCode
    pub fn execute_forward_in(
        &self,
        (rs1, src1): (RegName, RegData),
        (rs2, src2): (RegName, RegData),
    ) -> (RegData, RegData) {
        let MemAccessRegs {
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_m,
                    reg_write: reg_write_m,
                },
            alu_out: alu_out_m,
            write_reg: write_reg_m,
            ..
        } = self.tick_state.mem_access_regs;
        let WritebackRegs {
            wb_ctrl:
                WritebackControl {
                    mem_to_reg: mem_to_reg_w,
                    reg_write: reg_write_w,
                },
            alu_out: alu_out_w,
            read_data: read_data_w,
            write_reg: write_reg_w,
            ..
        } = self.tick_state.writeback_regs;

        let forward_single = |r: RegName, src: RegData| {
            if reg_eq_non_zero(r, write_reg_m) && reg_write_m && !mem_to_reg_m {
                alu_out_m
            } else if reg_eq_non_zero(r, write_reg_w) && reg_write_w {
                ExecutionEngine::writeback_mutex(mem_to_reg_w, alu_out_w, read_data_w)
            } else {
                src
            }
        };

        let src_a = forward_single(rs1, src1);
        let src_b = forward_single(rs2, src2);
        (src_a, src_b)
    }

    pub fn issue_jump(&mut self) {
        self.decode_regs.clr(true, ClockCycle::SecondHalf);
        self.issue_regs.clr(true, ClockCycle::SecondHalf);
    }
}

fn reg_eq_non_zero(r1: RegName, r2: RegName) -> bool {
    r1 == r2 && !r1.is_zero()
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct TickState {
    fetch_regs: FetchRegs,
    decode_regs: DecodeRegs,
    issue_regs: IssueRegs,
    execute_regs: ExecuteRegs,
    execute_out: Option<MemAccessRegs>,
    mem_access_regs: MemAccessRegs,
    mem_access_out: Option<WritebackRegs>,
    writeback_regs: WritebackRegs,
}
