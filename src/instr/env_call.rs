use super::instr_mod;
use crate::components::cpu::reg::RegData;

use std::error::Error;
use std::fmt;

instr_mod!(ecall);
instr_mod!(ebreak);

macro_rules! system_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        impl crate::instr::decode::Decode for $name {
            fn decode(_: crate::instr::raw::RawInstr) -> Self
            where
                Self: Sized,
            {
                Self
            }
        }

        impl crate::instr::Instr for $name {}

        impl crate::instr::stall::StallControl for $name {
            fn read_regs(
                &self,
            ) -> std::collections::HashSet<crate::components::cpu::reg::arf::ArchRegName> {
                let regs: [_; crate::components::cpu::reg::arf::ArchRegFile::SIZE] = std::array::from_fn(|reg| {
                    reg.try_into().unwrap()
                });
                std::collections::HashSet::from(regs)
            }

            fn write_reg(&self) -> Option<crate::components::cpu::reg::arf::ArchRegName> {
                None
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = crate::instr::display_width!(f);
                write!(f, "{:<width$}", Self::DISPLAY_NAME)
            }
        }
    };
}
use system_instr;

//#region Syscall

macro_rules! from_into_syscall {
    ($( $code:path => $n:literal, )*) => {
        impl From<SyscallCode> for u32 {
            fn from(value: SyscallCode) -> Self {
                match value {
                    $( $code => $n, )*
                }
            }
        }

        impl TryFrom<u32> for SyscallCode {
            type Error = SyscallConversionError;
        
            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $( $n => Ok($code), )*
                    code => Err(SyscallConversionError(code)),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyscallCode {
    Exit,
    Close,
    Lseek,
    Read,
    Sbrk,
    Write,
}

from_into_syscall! {
    SyscallCode::Exit => 93,
    SyscallCode::Close => 57,
    SyscallCode::Lseek => 62,
    SyscallCode::Read => 63,
    SyscallCode::Sbrk => 214,
    SyscallCode::Write => 64,
}

impl TryFrom<RegData> for SyscallCode {
    type Error = SyscallConversionError;

    fn try_from(value: RegData) -> Result<Self, Self::Error> {
        let value = value as u32;
        Self::try_from(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallConversionError(u32);

impl fmt::Display for SyscallConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid/supported system call code", self.0)
    }
}

impl Error for SyscallConversionError {}

//#endregion
