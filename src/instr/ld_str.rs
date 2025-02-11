use super::decode::{ITypeFormat, STypeFormat};
use super::stall::StallControl;
use super::Immediate;
use crate::components::cpu::reg::arf::ArchRegName;

use std::collections::HashSet;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Load<T>(ITypeFormat, PhantomData<T>);

impl<T> Load<T> {
    pub fn dest(&self) -> ArchRegName {
        self.0.rd
    }

    pub fn base(&self) -> ArchRegName {
        self.0.rs1
    }

    pub fn offset(&self) -> Immediate {
        self.0.imm
    }
}

impl<T> From<ITypeFormat> for Load<T> {
    fn from(value: ITypeFormat) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> StallControl for Load<T> {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        self.0.read_regs()
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        self.0.write_reg()
    }
}

impl<T> fmt::Display for Load<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dest().fmt(f)?;
        write!(f, ", {}({})", self.offset(), self.base())
    }
}

macro_rules! load_instr {
    ($name:ident<$size:ty>) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::ld_str::Load<$size>);

        impl crate::instr::Instr for $name {}

        impl From<crate::instr::decode::ITypeFormat> for $name {
            fn from(value: crate::instr::decode::ITypeFormat) -> Self {
                Self(value.into())
            }
        }

        crate::instr::impl_execute!($name, |&self, reg_file, _, _, load_agu, _| {
            let base = reg_file.get(self.0.base());
            let addr = load_agu.addr(base.get(), self.0.offset());
            Ok(crate::instr::execute::ExecuteResult::LoadAgu(
                self.0.dest(),
                addr,
            ))
        });

        impl crate::instr::mem_access::MemoryAccess for $name {
            fn load(
                &self,
                mem: &crate::components::memory::Memory,
                address: crate::components::memory::Address,
            ) -> Result<
                crate::components::cpu::reg::RegData,
                crate::components::cpu::error::MemAccessError,
            > {
                use crate::components::memory::MemoryAccess;
                let data: $size = mem.get(address);
                Ok(data as crate::components::cpu::reg::RegData)
            }
        }
    };
}
use load_instr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Store<T>(STypeFormat, PhantomData<T>);

impl<T> Store<T> {
    pub fn base(&self) -> ArchRegName {
        self.0.rs1
    }

    pub fn src(&self) -> ArchRegName {
        self.0.rs2
    }

    pub fn offset(&self) -> Immediate {
        self.0.imm
    }
}

impl<T> From<STypeFormat> for Store<T> {
    fn from(value: STypeFormat) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> StallControl for Store<T> {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        self.0.read_regs()
    }

    fn write_reg(&self) -> Option<ArchRegName> {
        self.0.write_reg()
    }
}

impl<T> fmt::Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.src().fmt(f)?;
        write!(f, ", {}({})", self.offset(), self.base())
    }
}

macro_rules! store_instr {
    ($name:ident<$size:ty>) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::ld_str::Store<$size>);

        impl crate::instr::Instr for $name {}

        impl From<crate::instr::decode::STypeFormat> for $name {
            fn from(value: crate::instr::decode::STypeFormat) -> Self {
                Self(value.into())
            }
        }

        crate::instr::impl_execute!($name, |&self, reg_file, _, _, _, store_agu| {
            let src = reg_file.get(self.0.src());
            let base = reg_file.get(self.0.base());
            let addr = store_agu.addr(base.get(), self.0.offset());
            Ok(crate::instr::execute::ExecuteResult::StoreAgu(
                addr,
                src.get(),
            ))
        });

        impl crate::instr::mem_access::MemoryAccess for $name {
            fn store(
                &self,
                mem: &mut crate::components::memory::Memory,
                address: crate::components::memory::Address,
                data: crate::components::cpu::reg::RegData,
            ) -> Result<(), crate::components::cpu::error::MemAccessError> {
                use crate::components::memory::MemoryAccess;
                mem.set(address, data as $size);
                Ok(())
            }
        }
    };
}
use store_instr;
