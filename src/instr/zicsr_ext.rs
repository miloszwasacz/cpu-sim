use super::instr_mod;

instr_mod!(csrrw);
instr_mod!(csrrs);
instr_mod!(csrrc);
instr_mod!(csrrwi);
instr_mod!(csrrsi);
instr_mod!(csrrci);

macro_rules! zicsr_ext_instr {
    ($name:ident, Reg, $ctrl:expr) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name(crate::instr::decode::encoding::CsrTypeFormat);

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Csr {
                    ctrl: $ctrl,
                    csr: value.0.csr(),
                    src: crate::instr::CsrSrc::Reg(value.0.rs1()),
                    dest: value.0.rd(),
                }
            }
        }
    };
    ($name:ident, Imm, $ctrl:expr) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name(crate::instr::decode::encoding::CsriTypeFormat);

        impl From<$name> for crate::instr::Instruction {
            fn from(value: $name) -> Self {
                crate::instr::Instruction::Csr {
                    ctrl: $ctrl,
                    csr: value.0.csr(),
                    src: crate::instr::CsrSrc::Imm(value.0.imm()),
                    dest: value.0.rd(),
                }
            }
        }
    };
}
use zicsr_ext_instr;
