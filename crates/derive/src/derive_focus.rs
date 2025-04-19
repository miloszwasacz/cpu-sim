use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Meta, Variant};

pub fn derive_focus(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);
    let name = &item.ident;
    let to_match_arm = |(from, to): (&Variant, &Variant)| {
        let from = &from.ident;
        let to = &to.ident;
        quote! { Self::#from => Self::#to, }
    };

    let next = item
        .variants
        .iter()
        .zip(
            item.variants
                .iter()
                .skip(1)
                .chain(item.variants.iter().take(1)),
        )
        .map(to_match_arm)
        .collect::<Vec<_>>();

    let prev = item
        .variants
        .iter()
        .zip(
            item.variants
                .iter()
                .rev()
                .take(1)
                .chain(item.variants.iter().take(item.variants.len() - 1)),
        )
        .map(to_match_arm)
        .collect::<Vec<_>>();

    let none_var = item.variants.iter().find_map(|v| {
        v.attrs.iter().find_map(|attr| match &attr.meta {
            Meta::Path(path) if path.is_ident("none") => Some(&v.ident),
            _ => None,
        })
    });

    let default_impl = match none_var {
        Some(default) => quote! {
            impl core::default::Default for #name {
                fn default() -> Self {
                    Self::#default
                }
            }
        },
        None => quote! {},
    };

    TokenStream::from(quote! {
        impl crate::ui::Focus for #name {
            fn next(self) -> Self {
                match self {
                    #( #next )*
                }
            }

            fn prev(self) -> Self {
                match self {
                    #( #prev )*
                }
            }
        }

        #default_impl
    })
}
