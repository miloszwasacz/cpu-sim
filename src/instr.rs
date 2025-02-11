use self::decode::*;
use self::execute::Execute;
use self::mem_access::MemoryAccess;
use self::raw::RawInstr;
use self::stall::StallControl;
use crate::include_generated;

use std::any::Any;
use std::fmt;
use std::rc::Rc;

macro_rules! instr_mod {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}
use instr_mod;

macro_rules! impl_execute {
    ($name:ty, |&$self:ident, $reg_file:tt, $pc:tt, $alu:tt, $load_agu:tt, $store_agu:tt| {
        $( $body:tt )*
    }) => {
        impl crate::instr::execute::Execute for $name {
            fn execute(
                &$self,
                $reg_file: &dyn crate::components::cpu::reg::RegFile<Index = crate::components::cpu::reg::arf::ArchRegName>,
                $pc: &mut crate::components::cpu::ProgramCounter,
                $alu: &mut crate::components::cpu::Alu,
                $load_agu: &mut crate::components::cpu::Agu,
                $store_agu: &mut crate::components::cpu::Agu,
            ) -> Result<crate::instr::execute::ExecuteResult, crate::components::cpu::error::ExecuteError>
            {
                $( $body )*
            }
        }
    };
}
use impl_execute;

macro_rules! impl_mem_access {
    ($name:ty) => {
        impl crate::instr::mem_access::MemoryAccess for $name {}
    };
}
use impl_mem_access;

instr_mod!(ctrl_trans);
instr_mod!(env_call);
instr_mod!(int_comput);
instr_mod!(ld_str);
instr_mod!(mem_ord);

pub mod decode;
pub mod execute;
pub mod mem_access;
pub mod raw;
pub mod stall;

pub trait Instr:
    Any + fmt::Debug + fmt::Display + Decode + Execute + MemoryAccess + StallControl
{
}

pub type Immediate = i32;

//#region Decode

impl RawInstr {
    pub(crate) fn decode(self) -> Result<Rc<dyn Instr>, Self> {
        decode(self)
    }
}

include_generated!("from_format_impls.rs");

//#endregion

//#region Display

const DISPLAY_PRETTY_WIDTH: usize = 9;

macro_rules! display_width {
    ($formatter:expr) => {
        if $formatter.alternate() {
            crate::instr::DISPLAY_PRETTY_WIDTH
        } else {
            0
        }
    };
}
use display_width;

include_generated!("display_impls.rs");

//#endregion
