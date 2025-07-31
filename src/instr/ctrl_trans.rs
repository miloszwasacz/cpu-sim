use super::instr_mod;

instr_mod!(jal);
instr_mod!(jalr);

instr_mod!(beq);
instr_mod!(bge);
instr_mod!(bgeu);
instr_mod!(blt);
instr_mod!(bltu);
instr_mod!(bne);

macro_rules! cond_branch {
    ($name:ident, $branch:ident) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name(crate::instr::decode::encoding::BTypeFormat);

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Branch {
                    ctrl: crate::instr::Branch::$branch,
                    src1: value.0.rs1(),
                    src2: value.0.rs2(),
                    offset: value.0.imm(),
                }
            }
        }
    };
}
use cond_branch;
