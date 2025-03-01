use crate::components::cpu::alu::AluControl;
use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::decode::{ITypeFormat, STypeFormat};
use crate::instr::execute::{AluSrcB, ExecUnit};
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Execute, Immediate, Issue, Writeback};

use std::fmt;
use std::marker::PhantomData;

instr_mod!(lb);
instr_mod!(lh);
instr_mod!(lw);
instr_mod!(lbu);
instr_mod!(lhu);
instr_mod!(sb);
instr_mod!(sh);
instr_mod!(sw);

//#region Macros

macro_rules! load_store_instr {
    ($name:ident, Load<$size:ty>) => {
        load_store_instr!($name, Load, $size);

        impl crate::instr::MemoryAccess for $name {
            fn mem_read(&self) -> Option<crate::instr::mem_access::MemRead> {
                fn read(
                    mem: &crate::components::memory::Memory,
                    addr: crate::components::memory::Address,
                ) -> crate::components::cpu::reg::RegData {
                    let data: $size = crate::components::memory::MemoryAccess::get(mem, addr);
                    data.into()
                }
                Some(read)
            }

            fn mem_write(&self) -> Option<crate::instr::mem_access::MemWrite> {
                None
            }
        }
    };
    ($name:ident, Store<$size:ty>) => {
        load_store_instr!($name, Store, $size);

        impl crate::instr::MemoryAccess for $name {
            fn mem_read(&self) -> Option<crate::instr::mem_access::MemRead> {
                None
            }

            fn mem_write(&self) -> Option<crate::instr::mem_access::MemWrite> {
                fn write(
                    mem: &mut crate::components::memory::Memory,
                    addr: crate::components::memory::Address,
                    data: crate::components::cpu::reg::RegData,
                ) {
                    let data = data.i() as $size;
                    crate::components::memory::MemoryAccess::set(mem, addr, data);
                }

                Some(write)
            }
        }
    };
    ($name:ident, $instr:ident, $size:ty) => {
        #[derive(
            Debug,
            cpu_sim_derive::Display,
            Clone,
            Copy,
            PartialEq,
            Eq,
            cpu_sim_derive::Decode,
            cpu_sim_derive::Issue,
            cpu_sim_derive::Writeback,
            cpu_sim_derive::Instr,
        )]
        pub struct $name(crate::instr::ld_str::$instr::<$size>);

        impl crate::instr::Execute for $name {
            delegate::delegate! {
                to self.0 {
                    fn exec_unit(&self) -> crate::instr::execute::ExecUnit;
                    fn alu_src_a(&self) -> crate::instr::execute::AluSrcA;
                    fn alu_src_b(&self) -> crate::instr::execute::AluSrcB;
                    fn alu_control(&self) -> crate::components::cpu::alu::AluControl;
                    fn mask_jump_target(&self) -> bool;
                }
            }
        }
    };
}
use load_store_instr;

macro_rules! delegate_decode {
    ($instr:ident, $format:ident) => {
        impl<T> Decode for $instr<T> {
            fn decode(raw: RawInstr) -> Self {
                Self($format::decode(raw), PhantomData)
            }

            delegate::delegate! {
                to self.0 {
                    fn rs1(&self) -> ArchRegName;
                    fn rs2(&self) -> ArchRegName;
                    fn rd(&self) -> ArchRegName;
                    fn imm(&self) -> Immediate;
                }
            }
        }
    };
}

macro_rules! impl_issue {
    ($instr:ident) => {
        impl<T> Issue for $instr<T> {
            fn branch(&self) -> crate::instr::issue::Branch {
                crate::instr::issue::Branch::None
            }
        }
    };
}

//#endregion

//#region Load

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Load<T>(ITypeFormat, PhantomData<T>);

delegate_decode!(Load, ITypeFormat);

impl_issue!(Load);

impl<T> Execute for Load<T> {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::LoadAgu
    }

    fn alu_src_b(&self) -> AluSrcB {
        AluSrcB::Imm
    }

    fn alu_control(&self) -> AluControl {
        AluControl::Add
    }
}

impl<T> Writeback for Load<T> {
    fn reg_write(&self) -> bool {
        true
    }
}

impl<T> fmt::Display for Load<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd().fmt(f)?;
        write!(f, ", {}({})", self.imm(), self.rs1())
    }
}

//#endregion

//#region Store

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Store<T>(STypeFormat, PhantomData<T>);

delegate_decode!(Store, STypeFormat);

impl_issue!(Store);

impl<T> Execute for Store<T> {
    fn exec_unit(&self) -> ExecUnit {
        ExecUnit::StoreAgu
    }

    fn alu_src_b(&self) -> AluSrcB {
        AluSrcB::Imm
    }

    fn alu_control(&self) -> AluControl {
        AluControl::Add
    }
}

impl<T> Writeback for Store<T> {
    fn reg_write(&self) -> bool {
        false
    }
}

impl<T> fmt::Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rs2().fmt(f)?;
        write!(f, ", {}({})", self.imm(), self.rs1())
    }
}

//#endregion
