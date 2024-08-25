use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Token;
use crate::html;

syn::custom_punctuation!(ClosingToken, </);

pub(crate) struct ClosingTag {
    pub token: ClosingToken,
    pub name: Option<html::Name>,
    pub end: Token![>],
}

impl ToTokens for ClosingTag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.token.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.end.to_tokens(tokens);
    }
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            token: input.parse()?,
            name: if input.peek(Token![>]) {
                None
            } else {
                Some(input.parse()?)
            },
            end: input.parse()?,
        })
    }
}