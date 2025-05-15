use super::instr_mod;

instr_mod!(mul);
instr_mod!(mulh);
instr_mod!(mulhsu);
instr_mod!(mulhu);
instr_mod!(mulw);

instr_mod!(div);
instr_mod!(divu);
instr_mod!(rem);
instr_mod!(remu);
instr_mod!(divw);
instr_mod!(divuw);
instr_mod!(remw);
instr_mod!(remuw);

macro_rules! m_ext_instr {
    ($name:ident, $mul_ctrl:ident) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name(crate::instr::decode::encoding::RTypeFormat);

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Mul {
                    ctrl: crate::instr::MulControl::$mul_ctrl,
                    src1: value.0.rs1(),
                    src2: value.0.rs2(),
                    dest: value.0.rd(),
                }
            }
        }
    };
}
use m_ext_instr;
