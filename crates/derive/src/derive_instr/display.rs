use super::InstrDeriveInput;
use crate::*;

use syn::Ident;

const DISPLAY_NAME_CONST: &str = "DISPLAY_NAME";

pub fn derive_display(input: TokenStream) -> TokenStream {
    let InstrDeriveInput {
        name, format, span, ..
    } = parse_macro_input!(input as InstrDeriveInput);
    let display_name_const = Ident::new(DISPLAY_NAME_CONST, span);

    TokenStream::from(quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = crate::instr::display::display_width!(f);
                write!(f, "{:<width$}", Self::#display_name_const)?;
                #format.fmt(f)
            }
        }
    })
}
