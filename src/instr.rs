use self::decode::Decode;
use self::execute::Execute;
use self::issue::Issue;
use self::mem_access::MemoryAccess;
use self::writeback::Writeback;
use crate::components::cpu::error::Exception;

use std::fmt;

macro_rules! instr_mod {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}
use instr_mod;

instr_mod!(ctrl_trans);
instr_mod!(env_call);
instr_mod!(int_comput);
instr_mod!(ld_str);
instr_mod!(mem_ord);

pub mod decode;
pub mod execute;
pub mod issue;
pub mod mem_access;
pub mod raw;
pub mod writeback;

pub trait Instr:
    fmt::Debug + fmt::Display + Decode + Issue + Execute + MemoryAccess + Writeback
{
}

pub type Immediate = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvTrap {
    Syscall,
    Break,
    Exception(Exception),
}

pub(crate) mod display {
    #[allow(unused_imports)]
    use super::*;
    use crate::include_generated;

    pub(crate) const DISPLAY_PRETTY_WIDTH: usize = 9;

    macro_rules! display_width {
        ($formatter:expr) => {
            if $formatter.alternate() {
                crate::instr::display::DISPLAY_PRETTY_WIDTH
            } else {
                0
            }
        };
    }
    pub(crate) use display_width;

    include_generated!("display_name_impls.rs");
}
