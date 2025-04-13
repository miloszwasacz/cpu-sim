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
        crate::instr::int_comput::int_comput_instr!(
            $name,
            crate::instr::decode::encoding::RTypeFormat
        );

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Alu {
                    ctrl: crate::instr::AluControl::$alu_ctrl,
                    src1: crate::instr::AluSrcA::Reg(value.0.rs1()),
                    src2: crate::instr::AluSrcB::Reg(value.0.rs2()),
                    dest: value.0.rd(),
                }
            }
        }
    };
    ($name:ident, Imm, $alu_ctrl:ident) => {
        crate::instr::int_comput::int_comput_instr!(
            $name,
            crate::instr::decode::encoding::ITypeFormat
        );

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Alu {
                    ctrl: crate::instr::AluControl::$alu_ctrl,
                    src1: crate::instr::AluSrcA::Reg(value.0.rs1()),
                    src2: crate::instr::AluSrcB::Imm(value.0.imm()),
                    dest: value.0.rd(),
                }
            }
        }
    };
    ($name:ident, $format:path) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name($format);
    };
}
use int_comput_instr;
