use super::EnvTrap;
use crate::components::cpu::alu::AluControl;

pub trait Execute {
    fn exec_unit(&self) -> ExecUnit;

    // ALU
    fn alu_src_a(&self) -> AluSrcA {
        Default::default()
    }
    fn alu_src_b(&self) -> AluSrcB;
    fn alu_control(&self) -> AluControl;

    // Jump/Branch
    fn mask_jump_target(&self) -> bool {
        false
    }

    // Env
    fn env_trap(&self) -> Option<EnvTrap> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecUnit {
    #[default]
    Alu,
    LoadAgu,
    StoreAgu,
    Branch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AluSrcA {
    #[default]
    Reg,
    Pc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AluSrcB {
    #[default]
    Reg,
    Imm,
}
