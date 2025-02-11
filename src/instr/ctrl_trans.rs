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
        pub struct $name(pub(in crate::instr) crate::instr::decode::BTypeFormat);

        impl $name {
            pub fn src1(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0 .0.rs1
            }

            pub fn src2(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                self.0 .0.rs2
            }

            pub fn offset(&self) -> crate::instr::Immediate {
                self.0 .0.imm
            }
        }

        impl crate::instr::Instr for $name {}
        
        crate::instr::impl_mem_access!($name);
    };
}
use cond_branch;

macro_rules! branch_execute_common {
    ($cmp:ident, $self:ident, $src1:expr, $src2:expr, $pc:expr, $alu:expr) => {{
        let base = $pc.read() as crate::components::cpu::reg::RegData;
        let target = $alu.add(base, $self.offset());

        let target = if $src1.$cmp(&$src2) {
            Some(target as crate::components::memory::Address)
        } else {
            None
        };
        Ok(crate::instr::execute::ExecuteResult::Branch(target))
    }};
}
use branch_execute_common;

macro_rules! impl_branch_execute {
    ($name:ty, $cmp:ident) => {
        impl_execute!($name, |&self, reg_file, pc, alu, _, _| {
            let src1 = reg_file.get(self.src1()).get();
            let src2 = reg_file.get(self.src2()).get();
            crate::instr::ctrl_trans::branch_execute_common!($cmp, self, src1, src2, pc, alu)
        });
    };
    ($name:ty, $cmp:ident, unsigned) => {
        impl_execute!($name, |&self, reg_file, pc, alu, _, _| {
            let src1 = reg_file.get(self.src1()).get_unsigned();
            let src2 = reg_file.get(self.src2()).get_unsigned();
            crate::instr::ctrl_trans::branch_execute_common!($cmp, self, src1, src2, pc, alu)
        });
    };
}
use impl_branch_execute;
