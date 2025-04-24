use crate::components::cpu::reg::{DataType, RegData};
use crate::components::memory::{Address, MemoryReadAccess, MemoryWriteAccess, L1D};
use crate::instr::decode::encoding::{ITypeFormat, STypeFormat};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;
use crate::instr::Instruction;

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

macro_rules! load_store_instr {
    ($name:ident, $parent:ty) => {
        #[derive(
            Debug, cpu_sim_derive::Display, Clone, Copy, PartialEq, Eq, cpu_sim_derive::Decode,
        )]
        pub struct $name($parent);

        impl From<$name> for crate::instr::Instruction {
            #[inline(always)]
            fn from(value: $name) -> Self {
                value.0.into()
            }
        }
    };
}
use load_store_instr;

//#region Load

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Load<T>(ITypeFormat, PhantomData<T>);

impl<T> Decode for Load<T> {
    fn decode(raw: RawInstr) -> Self {
        Self(ITypeFormat::decode(raw), PhantomData)
    }
}

impl<T: DataType> From<Load<T>> for Instruction {
    fn from(value: Load<T>) -> Self {
        let load = |mem: &mut L1D, addr: Address| {
            let data: T = MemoryReadAccess::read(mem, addr);
            data.into()
        };

        Instruction::Load {
            load,
            byte_count: size_of::<T>(),
            base: value.0.rs1(),
            offset: value.0.imm(),
            dest: value.0.rd(),
        }
    }
}

impl<T> fmt::Display for Load<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.rd().fmt(f)?;
        write!(f, ", {}({})", self.0.imm(), self.0.rs1())
    }
}

//#endregion

//#region Store

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Store<T>(STypeFormat, PhantomData<T>);

impl<T> Decode for Store<T> {
    fn decode(raw: RawInstr) -> Self {
        Self(STypeFormat::decode(raw), PhantomData)
    }
}

impl<T: DataType + Clone> From<Store<T>> for Instruction {
    fn from(value: Store<T>) -> Self {
        let store = |mem: &mut L1D, addr: Address, data: RegData| {
            let data: T = data.into();
            mem.write(addr, data);
        };

        Instruction::Store {
            store,
            byte_count: size_of::<T>(),
            src: value.0.rs2(),
            base: value.0.rs1(),
            offset: value.0.imm(),
        }
    }
}

impl<T> fmt::Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.rs2().fmt(f)?;
        write!(f, ", {}({})", self.0.imm(), self.0.rs1())
    }
}

//#endregion
