use self::agu::Agu;
use self::alu::Alu;
use self::branch::BranchUnit;
use super::circuit::{Circuit, ClockCycle};
use super::hazard::HazardUnit;
use super::reg::arf::ArchRegFile;
use super::reg::pipeline::{
    ErrorControl, ExecuteControl, ExecuteRegs, IssueControl, IssueRegs, Jump, MemAccessRegs,
    PipelineRegs, WritebackControl, WritebackRegs,
};
use super::reg::{RegData, RegFile};
use crate::instr::execute::{AluSrcA, AluSrcB, ExecUnit};
use crate::instr::EnvTrap;

pub mod agu;
pub mod alu;
pub mod branch;

pub struct ExecutionEngine {
    // Issue
    issue_regs: PipelineRegs<IssueRegs>,
    int_reg_file: Circuit<ArchRegFile>,
    execute_regs: PipelineRegs<ExecuteRegs>,

    // Execute
    alu: Circuit<Alu>,
    load_agu: Circuit<Agu>,
    store_agu: Circuit<Agu>,
    branch_unit: Circuit<BranchUnit>,
    mem_access_regs: PipelineRegs<MemAccessRegs>,

    // Writeback
    writeback_regs: Option<PipelineRegs<WritebackRegs>>,
}

impl ExecutionEngine {
    const WB_INIT_ERR: &'static str = "`writeback` should be initialized";

    // TODO mention connecting `writeback_regs` in the docs
    pub(super) fn new(issue_regs: &PipelineRegs<IssueRegs>) -> Self {
        // Issue
        let issue_regs = issue_regs.clone();
        let int_reg_file = Default::default();
        let execute_regs = Default::default();

        // Execute
        let alu = Circuit::new(Alu::new());
        let load_agu = Circuit::new(Agu::new());
        let store_agu = Circuit::new(Agu::new());
        let branch_unit = Circuit::new(BranchUnit::new());
        let mem_access_regs = Default::default();

        Self {
            // Issue
            issue_regs,
            int_reg_file,
            execute_regs,

            // Execute
            alu,
            load_agu,
            store_agu,
            branch_unit,
            mem_access_regs,

            // Writeback
            writeback_regs: None,
        }
    }

    pub(super) fn connect_writeback_regs(&mut self, writeback_regs: &PipelineRegs<WritebackRegs>) {
        self.writeback_regs = Some(writeback_regs.clone());
    }

    pub fn execute_regs(&self) -> &PipelineRegs<ExecuteRegs> {
        &self.execute_regs
    }

    pub fn mem_access_regs(&self) -> &PipelineRegs<MemAccessRegs> {
        &self.mem_access_regs
    }

    // TODO Add docs why it's unsafe
    pub(super) unsafe fn int_reg_file(&mut self) -> &mut ArchRegFile {
        unsafe { self.int_reg_file.inner_mut() }
    }

    pub fn start_cycle(&mut self) {
        self.int_reg_file.reset();
        self.execute_regs.reset();
        self.alu.reset();
        self.load_agu.reset();
        self.store_agu.reset();
        self.branch_unit.reset();
        self.mem_access_regs.reset();
    }

    pub fn issue(&mut self, hazard_unit: &mut HazardUnit) {
        let IssueRegs {
            pc_ctrl,
            is_ctrl,
            mut ex_ctrl,
            mem_ctrl,
            wb_ctrl,
            err_ctrl,
            rs1,
            rs2,
            rd,
            imm,
        } = self.issue_regs.read(ClockCycle::FirstHalf);
        let IssueControl { branch } = is_ctrl;
        let int_reg_file = self.int_reg_file.read(ClockCycle::SecondHalf);

        let src1 = (rs1, int_reg_file.get(rs1));
        let src2 = (rs2, int_reg_file.get(rs2));
        let (br_src1, br_src2) = hazard_unit.issue_forward_in(src1, src2);
        ex_ctrl.jump = branch.jumps(br_src1, br_src2);
        let write_reg = rd;

        let execute_regs = ExecuteRegs {
            pc_ctrl,
            ex_ctrl,
            mem_ctrl,
            wb_ctrl,
            err_ctrl,
            src1,
            src2,
            write_reg,
            imm,
        };

        self.execute_regs
            .write(ClockCycle::SecondHalf, execute_regs);

        if ex_ctrl.jump {
            hazard_unit.issue_jump();
        }
    }

    pub fn execute(&mut self, hazard_unit: &mut HazardUnit) -> (Jump, Option<EnvTrap>) {
        let ExecuteRegs {
            pc_ctrl,
            ex_ctrl,
            mem_ctrl,
            wb_ctrl,
            mut err_ctrl,
            src1,
            src2,
            write_reg,
            imm,
        } = self.execute_regs.read(ClockCycle::FirstHalf);
        let ExecuteControl {
            exec_unit,
            alu_src_a,
            alu_src_b,
            alu_control,
            jump,
            mask_jump_target,
            env_trap,
        } = ex_ctrl;
        let (src1, src2) = hazard_unit.execute_forward_in(src1, src2);

        let mut jump_target = None;
        let alu_out = match exec_unit {
            ExecUnit::Alu => {
                let src_a = match alu_src_a {
                    AluSrcA::Reg => src1,
                    AluSrcA::Pc => RegData::address(pc_ctrl.pc),
                };
                let src_b = match alu_src_b {
                    AluSrcB::Reg => src2,
                    AluSrcB::Imm => RegData::signed(imm),
                };
                let alu = self.alu.write(ClockCycle::FirstHalf);

                alu.process(alu_control, src_a, src_b)
            }
            ExecUnit::LoadAgu => {
                let base = src1;
                let offset = imm;
                let agu = self.load_agu.write(ClockCycle::FirstHalf);

                agu.addr(base, offset)
            }
            ExecUnit::StoreAgu => {
                let base = src1;
                let offset = imm;
                let agu = self.store_agu.write(ClockCycle::FirstHalf);

                agu.addr(base, offset)
            }
            ExecUnit::Branch => {
                let base = match alu_src_a {
                    AluSrcA::Reg => src1,
                    AluSrcA::Pc => RegData::address(pc_ctrl.pc),
                };
                let offset = imm;
                let branch_unit = self.branch_unit.write(ClockCycle::FirstHalf);

                if jump {
                    let target = branch_unit.target(base, offset, mask_jump_target);
                    if target == pc_ctrl.pc {
                        err_ctrl.jump_to_self = true;
                    }
                    jump_target = Some(target);
                }
                RegData::address(pc_ctrl.pc_plus4)
            }
        };
        let write_data = src2;
        let mem_access_regs = MemAccessRegs {
            mem_ctrl,
            wb_ctrl,
            err_ctrl,
            alu_out,
            write_data,
            write_reg,
        };

        hazard_unit.execute_forward_out(mem_access_regs);
        self.mem_access_regs
            .write(ClockCycle::SecondHalf, mem_access_regs);

        (jump_target, env_trap)
    }

    pub fn writeback(&mut self) -> ErrorControl {
        let writeback_regs = self.writeback_regs.as_ref().expect(Self::WB_INIT_ERR);
        let WritebackRegs {
            wb_ctrl,
            err_ctrl,
            alu_out,
            read_data,
            write_reg,
        } = writeback_regs.read(ClockCycle::FirstHalf);
        let WritebackControl {
            mem_to_reg,
            reg_write,
        } = wb_ctrl;
        let int_reg_file = self.int_reg_file.write(ClockCycle::FirstHalf);

        let data = Self::writeback_mutex(mem_to_reg, alu_out, read_data);
        if reg_write {
            int_reg_file.set(write_reg, data);
        }

        err_ctrl
    }

    pub(super) fn writeback_mutex(
        mem_to_reg: bool,
        alu_out: RegData,
        read_data: RegData,
    ) -> RegData {
        if mem_to_reg {
            read_data
        } else {
            alu_out
        }
    }
}
