use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{Error, Token};
use crate::html;

pub(crate) struct OpeningTag {
    pub token: Token![<],
    pub name: html::Name,
    pub attrs: Vec<html::Attribute>,
    pub end_token: EndToken,
}

syn::custom_punctuation!(InlineEnding, />);

pub enum EndToken {
    Normal(Token![>]),
    Inline(InlineEnding),
}

impl ToTokens for OpeningTag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.token.to_tokens(tokens);
        self.name.to_tokens(tokens);
        tokens.append_all(&self.attrs);
        self.end_token.to_tokens(tokens);
    }
}

impl Parse for OpeningTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token = input.parse()
            .map_err(|e| Error::new(e.span(), format!("expected '<': {e}")))?;
        let name = input.parse()
            .map_err(|e| Error::new(e.span(), format!("expected element name: {e}")))?;
        let mut attrs = Vec::new();

        let end_token;
        loop {
            if let Ok(token) = input.parse::<EndToken>() {
                end_token = token;
                break;
            }
            attrs.push(input.parse()?);
        }

        Ok(Self {
            token,
            name,
            attrs,
            end_token,
        })
    }
}

impl ToTokens for EndToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            EndToken::Normal(item) => item.to_tokens(tokens),
            EndToken::Inline(item) => item.to_tokens(tokens),
        }
    }
}

impl Parse for EndToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Normal(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Inline(item))
        } else {
            Err(input.error("expected '>' or '/>'"))
        }
    }
}
