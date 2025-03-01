use super::InstrDeriveInput;
use crate::*;

pub fn derive_writeback(input: TokenStream) -> TokenStream {
    let InstrDeriveInput { name, format, .. } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::writeback::Writeback for #name {
            fn reg_write(&self) -> bool {
                #format.reg_write()
            }
        }
    })
}
