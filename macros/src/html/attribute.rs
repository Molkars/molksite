use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Token};
use crate::html;

pub(crate) struct Attribute {
    pub name: html::Name,
    pub value: Option<(Token![=], AttributeValue)>,
}

pub(crate) enum AttributeValue {
    Path(syn::Path),
    Lit(syn::Lit),
    Block(syn::Block),
    Embedded(html::Embedded),
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()
                .map_err(|e| Error::new(e.span(), format!("expected attribute name: {e}")))?,
            value: match input.parse() {
                Ok(token) => Some((token, input.parse()?)),
                Err(_) => None
            },
        })
    }
}

impl ToTokens for AttributeValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttributeValue::Path(item) => item.to_tokens(tokens),
            AttributeValue::Lit(item) => item.to_tokens(tokens),
            AttributeValue::Block(item) => item.to_tokens(tokens),
            AttributeValue::Embedded(item) => item.to_tokens(tokens),
        }
    }
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Path(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Lit(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Block(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Embedded(item))
        } else {
            Err(input.error("expected attribute value: ident, literal, block, or (expr)"))
        }
    }
}