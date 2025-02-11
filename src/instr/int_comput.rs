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
    ($name:ident, $alu_op:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::decode::ITypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rd
            }

            pub fn src(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rs1
            }

            pub fn imm(&self) -> crate::instr::Immediate {
                self.0.imm
            }
        }

        impl crate::instr::Instr for $name {}
        
        crate::instr::impl_execute!($name, |&self, reg_file, _, alu, _, _| {
            let src = reg_file.get(self.src());
            let data = alu.$alu_op(src.get(), self.imm());
            Ok(crate::instr::execute::ExecuteResult::Alu(self.dest(), data))
        });
        
        crate::instr::impl_mem_access!($name);
    };
}
use int_reg_imm_instr;

macro_rules! int_upper_imm_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::decode::UTypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rd
            }

            pub fn imm(&self) -> crate::instr::Immediate {
                self.0.imm
            }
        }

        impl crate::instr::Instr for $name {}
        
        crate::instr::impl_mem_access!($name);
    };
}
use int_upper_imm_instr;

macro_rules! int_reg_reg_op {
    ($name:ident, $alu_op:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::decode::RTypeFormat);

        impl $name {
            pub fn dest(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rd
            }

            pub fn src1(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rs1
            }

            pub fn src2(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0.rs2
            }
        }

        impl crate::instr::Instr for $name {}
        
        crate::instr::impl_execute!($name, |&self, reg_file, _, alu, _, _| {
            let src1 = reg_file.get(self.src1());
            let src2 = reg_file.get(self.src2());
            let data = alu.$alu_op(src1.get(), src2.get());
            Ok(crate::instr::execute::ExecuteResult::Alu(self.dest(), data))
        });
        
        crate::instr::impl_mem_access!($name);
    };
}
use int_reg_reg_op;
