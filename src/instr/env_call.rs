use super::instr_mod;
use crate::components::cpu::reg::RegData;
use crate::os::OsFnResult;

use std::error::Error;
use std::fmt;

instr_mod!(ecall);
instr_mod!(ebreak);

macro_rules! system_instr {
    ($name:ident, $trap:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(());

        impl crate::instr::decode::Decode for $name {
            fn decode(_raw: crate::instr::raw::RawInstr) -> Self {
                #[cfg(debug_assertions)]
                {
                    assert_eq!(_raw.encoding(), Self::ENCODING);
                }
                Self(())
            }
        }

        impl From<$name> for crate::instr::Instruction {
            fn from(_: $name) -> Self {
                crate::instr::Instruction::EnvTrap(crate::instr::EnvTrap::$trap)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = crate::instr::display::display_width!(f);
                write!(f, "{:<width$}", Self::DISPLAY_NAME)
            }
        }
    };
}
use system_instr;

//#region Syscall

macro_rules! from_into_syscall {
    ($( $code:path => $n:literal, )*) => {
        impl From<SyscallCode> for OsFnResult {
            fn from(value: SyscallCode) -> Self {
                match value {
                    $( $code => $n, )*
                }
            }
        }

        impl TryFrom<OsFnResult> for SyscallCode {
            type Error = SyscallConversionError;

            fn try_from(value: OsFnResult) -> Result<Self, Self::Error> {
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
    Fstat,
    Lseek,
    Read,
    Sbrk,
    Write,
}

from_into_syscall! {
    SyscallCode::Exit => 93,
    SyscallCode::Close => 57,
    SyscallCode::Fstat => 80,
    SyscallCode::Lseek => 62,
    SyscallCode::Read => 63,
    SyscallCode::Sbrk => 214,
    SyscallCode::Write => 64,
}

impl TryFrom<RegData> for SyscallCode {
    type Error = SyscallConversionError;

    fn try_from(value: RegData) -> Result<Self, Self::Error> {
        Self::try_from(value.i() as OsFnResult)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallConversionError(OsFnResult);

impl fmt::Display for SyscallConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid/supported system call code", self.0)
    }
}

impl Error for SyscallConversionError {}

//#endregion
