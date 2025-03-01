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
            Debug,
            cpu_sim_derive::Display,
            Clone,
            Copy,
            PartialEq,
            Eq,
            cpu_sim_derive::Decode,
            cpu_sim_derive::MemoryAccess,
            cpu_sim_derive::Writeback,
            cpu_sim_derive::Instr,
        )]
        pub struct $name(crate::instr::decode::BTypeFormat);

        impl crate::instr::issue::Issue for $name {
            fn branch(&self) -> crate::instr::issue::Branch {
                crate::instr::issue::Branch::$branch
            }
        }

        impl crate::instr::Execute for $name {
            fn exec_unit(&self) -> crate::instr::execute::ExecUnit {
                crate::instr::execute::ExecUnit::Branch
            }

            fn alu_src_a(&self) -> crate::instr::execute::AluSrcA {
                crate::instr::execute::AluSrcA::Pc
            }

            fn alu_src_b(&self) -> crate::instr::execute::AluSrcB {
                crate::instr::execute::AluSrcB::Imm
            }

            fn alu_control(&self) -> crate::components::cpu::alu::AluControl {
                crate::components::cpu::alu::AluControl::Add
            }
        }
    };
}
use cond_branch;
