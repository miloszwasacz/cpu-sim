use super::InstrDeriveInput;
use crate::*;

pub fn derive_mem_access(input: TokenStream) -> TokenStream {
    let InstrDeriveInput { name, .. } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::MemoryAccess for #name {
            fn mem_read(&self) -> core::option::Option<crate::instr::mem_access::MemRead> {
                None
            }

            fn mem_write(&self) -> core::option::Option<crate::instr::mem_access::MemWrite> {
                None
            }
        }
    })
}
