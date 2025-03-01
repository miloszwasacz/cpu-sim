use super::InstrDeriveInput;
use crate::*;

pub fn derive_instr(input: TokenStream) -> TokenStream {
    let InstrDeriveInput { name, .. } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::Instr for #name {}
    })
}
