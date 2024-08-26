use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use crate::html;

pub(crate) enum Name {
    Ident(html::Ident),
    Embedded(html::Embedded),
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Name::Ident(item) => item.to_tokens(tokens),
            Name::Embedded(item) => item.to_tokens(tokens),
        }
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Ident(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Embedded(item))
        } else {
            Err(input.error("expected html identifier or a rust expression"))
        }
    }
}