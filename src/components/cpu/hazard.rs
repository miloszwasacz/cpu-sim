use super::circuit::ClockCycle;
use super::reg::arf::ArchRegName;
use super::reg::pipeline::{
    DecodeRegs, ExecuteControl, ExecuteRegs, FetchRegs, IssueControl, IssueRegs, MemAccessRegs,
    PipelineRegs, WritebackControl, WritebackRegs,
};
use super::reg::RegData;
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
            mem_access_regs: self.mem_access_regs.read(ClockCycle::FirstHalf),
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
            let stall_e = reg_write_e
                // TODO Uncomment when forwarding is implemented
                // && mem_to_reg_e
                && (reg_eq_non_zero(rs1_i, write_reg_e)
                    || reg_eq_non_zero(rs2_i, write_reg_e));

            // Operand for load in will be loaded from memory
            let stall_m = reg_write_m
                // TODO Uncomment when forwarding is implemented
                // && mem_to_reg_m
                && (reg_eq_non_zero(rs1_i, write_reg_m)
                    || reg_eq_non_zero(rs2_i, write_reg_m));

            stall_e || stall_m
        };
        let branch_stall = {
            // Operand for branch in Execute stage
            let branch_stall_e = reg_write_e
                && (reg_eq_non_zero(write_reg_e, rs1_i) || reg_eq_non_zero(write_reg_e, rs2_i));

            // Operand for branch will be loaded from memory
            let branch_stall_m = mem_to_reg_m
                && (reg_eq_non_zero(write_reg_m, rs1_i) || reg_eq_non_zero(write_reg_m, rs2_i));

            branch_i && (branch_stall_e || branch_stall_m)
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

    pub fn forward_issue(
        &self,
        (rs1, src1): (ArchRegName, RegData),
        (rs2, src2): (ArchRegName, RegData),
    ) -> (RegData, RegData) {
        let MemAccessRegs {
            wb_ctrl:
                WritebackControl {
                    reg_write: reg_write_m,
                    ..
                },
            alu_out: alu_out_m,
            write_reg: write_reg_m,
            ..
        } = self.tick_state.mem_access_regs;

        let forward_single = |r: ArchRegName, src: RegData| {
            // if Self::reg_eq_non_zero(r, write_reg_m) && reg_write_m {
            //     todo!("Forwarding not implemented");
            //     alu_out_m
            // } else {
            src
            // }
        };

        let src_a = forward_single(rs1, src1);
        let src_b = forward_single(rs2, src2);
        (src_a, src_b)
    }

    pub fn forward_execute(
        &self,
        (rs1, src1): (ArchRegName, RegData),
        (rs2, src2): (ArchRegName, RegData),
    ) -> (RegData, RegData) {
        let MemAccessRegs {
            wb_ctrl:
                WritebackControl {
                    reg_write: reg_write_m,
                    ..
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

        let forward_single = |r: ArchRegName, src: RegData| {
            if reg_eq_non_zero(r, write_reg_m) && reg_write_m {
                todo!("Forwarding not implemented");
                alu_out_m
            } else if reg_eq_non_zero(r, write_reg_w) && reg_write_w {
                todo!("Forwarding not implemented");
                if mem_to_reg_w {
                    read_data_w
                } else {
                    alu_out_w
                }
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

fn reg_eq_non_zero(r1: ArchRegName, r2: ArchRegName) -> bool {
    r1 == r2 && !r1.is_zero()
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct TickState {
    fetch_regs: FetchRegs,
    decode_regs: DecodeRegs,
    issue_regs: IssueRegs,
    execute_regs: ExecuteRegs,
    mem_access_regs: MemAccessRegs,
    writeback_regs: WritebackRegs,
}
