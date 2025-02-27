#![allow(non_snake_case)]

use proc_macro2::{Ident, Span};
use quote::quote;
use std::rc::Rc;
use syn::{
    self, parse::Parse, parse_macro_input, punctuated::Punctuated, Path,
    PathSegment, Token, Type, TypePath,
};

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

pub(self) fn lowercasify_the_type(ty: &Type) -> Type {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            let lower_ident = Ident::new(
                &ident.to_string().to_lowercase(),
                Span::call_site(),
            );

            let new_path = Path {
                leading_colon: path.leading_colon,
                segments: vec![PathSegment {
                    ident: lower_ident,
                    arguments: Default::default(),
                }]
                .into_iter()
                .collect(),
            };

            return Type::Path(TypePath {
                qself: None,
                path: new_path,
            });
        }
    }
    ty.clone()
}

/// .
pub(self) fn self_inner_join<T>(types_arr: &Vec<T>) -> Vec<Vec<(&T, &T)>> {
    let types_rc = Rc::new(types_arr);
    let row = types_rc.clone().into_iter();

    row.enumerate()
        .map(|(i, T)| {
            // let col = row.take(i).chain(row.skip(i + 1));
            let col = types_rc.clone().into_iter().take(i);
            let to_skip = col.clone().skip(i + 1);
            let skipped = col.chain(to_skip);

            skipped.map(move |U| (T, U)).collect()
        })
        .collect()
}

/// .
#[proc_macro]
pub fn casting_number(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let types: Vec<Type> = parse_macro_input!(input as Args).into();

    // pairs to make into each other
    let cart_prod = self_inner_join(&types).into_iter();

    let cast_prod: Vec<Vec<_>> = cart_prod
        .map(|v| {
            v.into_iter()
                .map(|(T, U)| {
                    let lower_ty = lowercasify_the_type(U);
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
