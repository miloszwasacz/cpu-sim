use super::instr_mod;

instr_mod!(addi);
instr_mod!(andi);
instr_mod!(ori);
instr_mod!(slti);
instr_mod!(sltiu);
instr_mod!(xori);

instr_mod!(slli);
instr_mod!(srai);
instr_mod!(srli);

instr_mod!(auipc);
instr_mod!(lui);

instr_mod!(add);
instr_mod!(and);
instr_mod!(or);
instr_mod!(sll);
instr_mod!(slt);
instr_mod!(sltu);
instr_mod!(sra);
instr_mod!(srl);
instr_mod!(sub);
instr_mod!(xor);

macro_rules! int_reg_imm_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::pipeline::decode::ITypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::reg::RegisterName {
                self.0.rd
            }

            pub fn src(&self) -> crate::reg::RegisterName {
                self.0.rs1
            }

            pub fn imm(&self) -> crate::instr::Immediate {
                self.0.imm
            }
        }
        
        impl crate::instr::Instr for $name {}
    };
}
use int_reg_imm_instr;

macro_rules! int_upper_imm_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::pipeline::decode::UTypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::reg::RegisterName {
                self.0.rd
            }

            pub fn imm(&self) -> crate::instr::Immediate {
                self.0.imm
            }
        }
        
        impl crate::instr::Instr for $name {}
    };
}
use int_upper_imm_instr;

macro_rules! int_reg_reg_op {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::pipeline::decode::RTypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::reg::RegisterName {
                self.0.rd
            }

            pub fn src1(&self) -> crate::reg::RegisterName {
                self.0.rs1
            }

            pub fn src2(&self) -> crate::reg::RegisterName {
                self.0.rs2
            }
        }
        
        impl crate::instr::Instr for $name {}
    };
}
use int_reg_reg_op;
