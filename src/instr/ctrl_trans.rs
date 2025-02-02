instr_mod!(jal);
instr_mod!(jalr);

instr_mod!(beq);
instr_mod!(bge);
instr_mod!(bgeu);
instr_mod!(blt);
instr_mod!(bltu);
instr_mod!(bne);

macro_rules! cond_branch {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::pipeline::decode::BTypeFormat);

        impl $name {
            pub fn src1(&self) -> crate::reg::RegisterName {
                self.0 .0.rs1
            }

            pub fn src2(&self) -> crate::reg::RegisterName {
                self.0 .0.rs2
            }

            pub fn offset(&self) -> crate::instr::Immediate {
                self.0 .0.imm
            }
        }

        impl crate::instr::Instr for $name {}
    };
}
use cond_branch;
