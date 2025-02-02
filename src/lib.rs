use std::any::Any;

pub mod instr;
pub mod pipeline;
pub mod reg;

macro_rules! include_generated {
    ($file:literal) => {
        include!(concat!(env!("OUT_DIR"), "/", $file));
    };
}
use include_generated;

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
