#![allow(non_snake_case)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parse, parse_macro_input, punctuated::Punctuated, Token, Type};

struct Args(pub Vec<Type>);
impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Type, Token![,]>::parse_terminated(input)?;
        Ok(Args(vars.into_iter().collect()))
    }
}

impl Into<Vec<Type>> for Args {
    fn into(self) -> Vec<Type> {
        self.0
    }
}

pub(self) fn self_inner_join<T>(t: Vec<T>) -> Vec<(T, T)> {
    todo!()
}

#[proc_macro]
pub fn cast_to(input: TokenStream) -> TokenStream {
    let types: Vec<Type> = parse_macro_input!(input as Args).into();

    // pairs to make into each other
    let cart_prod = types
        .iter()
        .zip(types.iter().rev())
        .into_iter()
        .flat_map(|(T, U)| {
            if quote!(#T).to_string() == quote!(#U).to_string() {
                None
            } else {
                Some((T, U))
            }
        })
        .into_iter();

    let cast_prod = cart_prod.map(|(T, U)| {
        quote! {
            impl From<#T> for #U {
                fn from(value: #T) -> Self {
                    Self(value.0 as $b)
                }
            }
        }
    });

    TokenStream::from(quote! {
        #(#types)*
    })
}
