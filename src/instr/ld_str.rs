use crate::instr::Immediate;
use crate::pipeline::decode::{ITypeFormat, STypeFormat};
use crate::reg::RegisterName;

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
    pub fn dest(&self) -> RegisterName {
        self.0.rd
    }

    pub fn base(&self) -> RegisterName {
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

impl<T> fmt::Display for Load<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#}, {}({})", self.dest(), self.offset(), self.base())
        } else {
            write!(f, "{}, {}({})", self.dest(), self.offset(), self.base())
        }
    }
}

macro_rules! load_instr {
    ($name:ident<$size:ty>) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::ld_str::Load<$size>);

        impl crate::instr::Instr for $name {}

        impl From<crate::pipeline::decode::ITypeFormat> for $name {
            fn from(value: crate::pipeline::decode::ITypeFormat) -> Self {
                Self(value.into())
            }
        }
    };
}
use load_instr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Store<T>(STypeFormat, PhantomData<T>);

impl<T> Store<T> {
    pub fn base(&self) -> RegisterName {
        self.0.rs1
    }

    pub fn src(&self) -> RegisterName {
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

impl<T> fmt::Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#}, {}({})", self.src(), self.offset(), self.base())
        } else {
            write!(f, "{}, {}({})", self.src(), self.offset(), self.base())
        }
    }
}

macro_rules! store_instr {
    ($name:ident<$size:ty>) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(in crate::instr) crate::instr::ld_str::Store<$size>);

        impl crate::instr::Instr for $name {}

        impl From<crate::pipeline::decode::STypeFormat> for $name {
            fn from(value: crate::pipeline::decode::STypeFormat) -> Self {
                Self(value.into())
            }
        }
    };
}
use store_instr;
