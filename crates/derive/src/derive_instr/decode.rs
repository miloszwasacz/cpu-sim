use super::InstrDeriveInput;
use crate::*;

pub fn derive_decode(input: TokenStream) -> TokenStream {
    let InstrDeriveInput {
        name,
        format_ty,
        ..
    } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::decode::Decode for #name {
            fn decode(raw: crate::instr::raw::RawInstr) -> Self {
                Self(<#format_ty>::decode(raw))
            }
        }
    })
}
