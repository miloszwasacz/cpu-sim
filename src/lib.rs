use std::any::Any;

pub mod components;
pub mod instr;
pub mod os;
pub mod pipeline;
pub mod reg;

macro_rules! include_generated {
    ($file:literal) => {
        include!(concat!(env!("OUT_DIR"), "/", $file));
    };
}
use include_generated;

macro_rules! int_impl {
    ($macro_name:tt $(, $arg:tt )*) => {
        $macro_name!(u8 $(, $arg )*);
        $macro_name!(u16 $(, $arg )*);
        $macro_name!(u32 $(, $arg )*);
        $macro_name!(u64 $(, $arg )*);
        $macro_name!(i8 $(, $arg )*);
        $macro_name!(i16 $(, $arg )*);
        $macro_name!(i32 $(, $arg )*);
        $macro_name!(i64 $(, $arg )*);
    };
}
use int_impl;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
