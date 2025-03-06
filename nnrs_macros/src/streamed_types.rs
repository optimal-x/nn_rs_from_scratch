use std::ops::Deref;

use syn::{Token, Type};

pub(crate) struct Args(pub Vec<Type>);

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars =
            syn::punctuated::Punctuated::<Type, Token![,]>::parse_terminated(
                input,
            )?;
        Ok(Args(vars.into_iter().collect()))
    }
}

impl Deref for Args {
    type Target = Vec<Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
