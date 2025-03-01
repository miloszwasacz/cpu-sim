use super::InstrDeriveInput;
use crate::*;

pub fn derive_issue(input: TokenStream) -> TokenStream {
    let InstrDeriveInput { name, .. } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::issue::Issue for #name {
            fn branch(&self) -> crate::instr::issue::Branch {
                crate::instr::issue::Branch::None
            }
        }
    })
}
