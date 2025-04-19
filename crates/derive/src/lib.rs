use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

//#region register_names!

mod reg_names;

#[proc_macro]
pub fn register_names(input: TokenStream) -> TokenStream {
    reg_names::register_names(input)
}

//#endregion

//#region Instruction derives

mod derive_instr;

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    derive_instr::decode::derive_decode(input)
}

#[proc_macro_derive(Display)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    derive_instr::display::derive_display(input)
}

//#endregion

//#region #[derive(Focus)]

mod derive_focus;

#[proc_macro_derive(Focus, attributes(none))]
pub fn derive_focus(input: TokenStream) -> TokenStream {
    derive_focus::derive_focus(input)
}

//#endregion
