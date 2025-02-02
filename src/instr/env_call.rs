use super::instr_mod;

instr_mod!(ecall);
instr_mod!(ebreak);

macro_rules! system_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
        
        impl crate::pipeline::decode::Decode for $name {
            fn decode(_: crate::instr::raw::RawInstr) -> Self
            where
                Self: Sized,
            {
                Self
            }
        }
        
        impl crate::instr::Instr for $name {}
        
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = crate::instr::display_width!(f);
                write!(f, "{:<width$}", Self::DISPLAY_NAME)
            }
        }
    };
}
use system_instr;