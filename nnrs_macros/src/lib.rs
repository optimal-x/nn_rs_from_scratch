#![allow(non_snake_case)]

pub(crate) mod streamed_types;
pub(crate) mod utils;

use quote::quote;
use streamed_types::Args;
use syn::parse_macro_input;

/// .
#[proc_macro]
pub fn cast_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let types = parse_macro_input!(input as Args);

    // pairs to make into each other
    let cart_prod = utils::self_inner_join(&types).into_iter();

    let cast_prod: Vec<Vec<_>> = cart_prod
        .map(|v| {
            v.into_iter()
                .map(|(T, U)| {
                    let lower_ty = utils::type_lowercasification(U);
                    quote! {
                        impl From<#T> for #U {
                            fn from(value: #T) -> Self {
                                Self(value.0 as #lower_ty)
                            }
                        }
                    }
                })
                .collect()
        })
        .collect();

    proc_macro::TokenStream::from(quote! {
        #(#(#cast_prod)*)*
    })
}

#[proc_macro]
pub fn impl_numfns(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let types: Vec<&str> = parse_macro_input!(input as Args)
        .iter()
        .flat_map(|t| {
            let str_type = stringify!(t);
            match str_type {
                "F32" | "F64" => Some(str_type),
                _ => None,
            }
        })
        .collect();

    let generated = quote! {
        impl NumberFuncs
    };

    todo!()
}
