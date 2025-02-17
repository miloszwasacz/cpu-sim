use self::reg_names::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod reg_names {
    use convert_case::{Case, Casing};
    use quote::quote;
    use syn::parse::{Parse, ParseStream};
    use syn::punctuated::Punctuated;
    use syn::{braced, Expr, Ident, Token, Type};

    mod kw {
        syn::custom_keyword!(reg);
    }

    pub struct RegisterNamesInput {
        pub ty: Type,
        pub regs: Punctuated<Register, Token![,]>,
    }

    pub struct Register(Ident, Expr);
    
    impl Register {
        pub fn definition_tokens(&self) -> proc_macro2::TokenStream {
            let Self(name, number) = self;
            quote! {
                pub const #name: Self = Self(#number);
            }
        }
        
        pub fn display_tokens(&self) -> proc_macro2::TokenStream {
            let Self(name, number) = self;
            let name = name.to_string().to_case(Case::Flat);
            quote! {
                #number => #name
            }
        }
    }

    impl Parse for RegisterNamesInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ty = input.parse()?;
            let content;
            braced!(content in input);
            let regs = content.parse_terminated(Register::parse, Token![,])?;
            Ok(Self { ty, regs })
        }
    }

    impl Parse for Register {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            input.parse::<kw::reg>()?;
            let name = input.parse()?;
            input.parse::<Token![=]>()?;
            let number = input.parse()?;
            Ok(Self(name, number))
        }
    }
}

#[proc_macro]
pub fn register_names(input: TokenStream) -> TokenStream {
    let RegisterNamesInput { ty, regs } = parse_macro_input!(input as RegisterNamesInput);
    let defs = regs.iter().map(|r| r.definition_tokens()).collect::<Vec<_>>();
    let displays = regs.iter().map(|r| r.display_tokens()).collect::<Vec<_>>();
    
    TokenStream::from(quote! {
        impl #ty {
            #( #defs )*
        }
        
        impl std::fmt::Display for #ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let width = if f.alternate() { 4 } else { 0 };
                write!(
                    f,
                    "{:>width$}",
                    match self.0 {
                        #( #displays, )*
                        _ => unreachable!(),
                    }
                )
            }
        }
    })
}
