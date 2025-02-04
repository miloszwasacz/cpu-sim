use crate::include_generated;
use crate::pipeline::decode::*;

use std::any::Any;
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
pub mod raw;

pub trait Instr: Any + fmt::Debug + fmt::Display + Decode {}

pub type Immediate = i32;

include_generated!("from_format_impls.rs");

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
