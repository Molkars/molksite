use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{token, Token};

enum NameLink {
    Ident(syn::Ident),
    Type(Token![type]),
    For(Token![for]),
}

impl PartialEq for NameLink {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Display for NameLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NameLink::Ident(v) => write!(f, "{v}"),
            NameLink::Type(_) => write!(f, "type"),
            NameLink::For(_) => write!(f, "for"),
        }
    }
}

impl ToTokens for NameLink {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            NameLink::Ident(item) => item.to_tokens(tokens),
            NameLink::Type(item) => item.to_tokens(tokens),
            NameLink::For(item) => item.to_tokens(tokens),
        }
    }
}

impl Parse for NameLink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Ident(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Type(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::For(item))
        } else {
            Err(input.error("invalid name token"))
        }
    }
}

pub(crate) struct Ident(Punctuated<NameLink, token::Minus>);

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