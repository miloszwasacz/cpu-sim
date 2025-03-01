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

macro_rules! int_comput_instr {
    ($name:ident, Reg, $alu_ctrl:ident) => {
        crate::instr::int_comput::int_comput_instr!($name, crate::instr::decode::RTypeFormat, Reg, $alu_ctrl);
    };
    ($name:ident, Imm, $alu_ctrl:ident) => {
        crate::instr::int_comput::int_comput_instr!($name, crate::instr::decode::ITypeFormat, Imm, $alu_ctrl);
    };
    ($name:ident, $format:path, $src_b:ident, $alu_ctrl:ident) => {
        #[derive(
            Debug, 
            cpu_sim_derive::Display, 
            Clone, 
            Copy, 
            PartialEq, 
            Eq, 
            cpu_sim_derive::Decode, 
            cpu_sim_derive::Issue, 
            cpu_sim_derive::MemoryAccess, 
            cpu_sim_derive::Writeback, 
            cpu_sim_derive::Instr,
        )]
        pub struct $name($format);

        impl crate::instr::Execute for $name {
            fn exec_unit(&self) -> crate::instr::execute::ExecUnit {
                crate::instr::execute::ExecUnit::Alu
            }

            fn alu_src_b(&self) -> crate::instr::execute::AluSrcB {
                crate::instr::execute::AluSrcB::$src_b
            }

            fn alu_control(&self) -> crate::components::cpu::alu::AluControl {
                crate::components::cpu::alu::AluControl::$alu_ctrl
            }
        }
    };
}
use int_comput_instr;
