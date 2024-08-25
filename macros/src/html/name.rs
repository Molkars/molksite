use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token;
use crate::html;

pub(crate) enum Name {
    Ident(html::Ident),
    Expr(NameExpr),
}

pub(crate) struct NameExpr {
    pub paren: token::Paren,
    pub expr: syn::Expr,
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Name::Ident(item) => item.to_tokens(tokens),
            Name::Expr(item) => item.to_tokens(tokens),
        }
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Ident(item))
        } else if let Ok(item) = input.parse() {
            Ok(Self::Expr(item))
        } else {
            Err(input.error("expected html identifier or a rust expression"))
        }
    }
}

impl ToTokens for NameExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}

impl Parse for NameExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: syn::parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}