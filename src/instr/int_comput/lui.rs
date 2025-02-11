use super::int_upper_imm_instr;
use crate::components::cpu::error::ExecuteError;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::components::cpu::reg::RegFile;
use crate::components::cpu::{Agu, Alu, ProgramCounter};
use crate::instr::execute::{Execute, ExecuteResult};

int_upper_imm_instr!(Lui);

impl Execute for Lui {
    fn execute(
        &self,
        _: &dyn RegFile<Index = ArchRegName>,
        _: &mut ProgramCounter,
        _: &mut Alu,
        _: &mut Agu,
        _: &mut Agu,
    ) -> Result<ExecuteResult, ExecuteError> {
        Ok(ExecuteResult::Alu(self.dest(), self.imm()))
    }
}
