use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;

pub(crate) struct Ident(Punctuated<syn::Ident, token::Minus>);

impl Parse for Ident {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Punctuated::parse_separated_nonempty(input).map(Self)
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "-")?;
            }
            write!(f, "{v}")?;
        }
        Ok(())
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().zip(other.0.iter())
            .all(|(a, b)| a == b)
    }
}