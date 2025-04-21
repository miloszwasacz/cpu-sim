pub mod components;
pub mod instr;
pub mod os;
pub mod config;

/// Includes a file generated during build.
macro_rules! include_generated {
    ($file:literal) => {
        include!(concat!(env!("OUT_DIR"), "/", $file));
    };
}
use include_generated;

//TODO Example usage
/// Applies the specified macro to all the given types with the provided args.
macro_rules! apply_macro {
    ($( $ty:ty )+ => $macro_name:ident!($( $arg:tt, )*)) => {
        macro_rules! __private_generated_macro_calls {
            ( $arg0:ty ) => {
                $macro_name!($arg0 $(, $arg )*);
            };
        }

        $(
            __private_generated_macro_calls!($ty);
        )+
    };
    ($( $ty:ty )+ => $macro_name:ident!($arg0:tt $(, $arg:tt)*)) => {
        crate::apply_macro!($( $ty )+ => $macro_name!($arg0, $( $arg, )*));
    };
}
use apply_macro;

const BITS_IN_BYTE: usize = 8;
const IALIGN: usize = 32;
