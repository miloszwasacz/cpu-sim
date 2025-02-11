use super::int_upper_imm_instr;
use crate::components::cpu::error::ExecuteError;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::components::cpu::reg::{RegData, RegFile};
use crate::components::cpu::{Agu, Alu, ProgramCounter};
use crate::instr::execute::{Execute, ExecuteResult};

int_upper_imm_instr!(Auipc);

impl Execute for Auipc {
    fn execute(
        &self,
        _: &dyn RegFile<Index = ArchRegName>,
        pc: ProgramCounter,
        alu: &mut Alu,
        _: &mut Agu,
        _: &mut Agu,
    ) -> Result<ExecuteResult, ExecuteError> {
        let base = pc.read() as RegData;
        let addr = alu.add(base, self.imm());
        Ok(ExecuteResult::Alu(self.dest(), addr))
    }
}
