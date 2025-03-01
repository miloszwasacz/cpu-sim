use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parse_quote, parse_quote_spanned, Data, DeriveInput, Expr, Field, Ident, Type,
};

pub mod decode;
pub mod display;
pub mod instr;
pub mod issue;
pub mod mem_access;
pub mod writeback;

struct InstrDeriveInput {
    name: Ident,
    format: Expr,
    format_ty: Type,
    span: Span,
}

impl Parse for InstrDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = input.parse()?;
        let span = input.span();

        let name = input.ident;
        match input.data {
            Data::Struct(s) => match s.fields.into_iter().next() {
                Some(Field { ident, ty, .. }) => {
                    let format = match ident {
                        None => parse_quote! { self.0 },
                        Some(ident) => parse_quote_spanned! { ident.span() => self.#ident },
                    };
                    let format_ty = ty;
                    Ok(Self {
                        name,
                        format,
                        format_ty,
                        span,
                    })
                }
                None => Err(syn::Error::new(
                    span,
                    "Only structs with at least one field can derive this trait",
                )),
            },
            _ => Err(syn::Error::new(span, "Only structs can derive this trait")),
        }
    }
}
