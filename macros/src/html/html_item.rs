use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token;
use crate::html::Element;

pub(crate) enum Item {
    Element(Element),
    Expr(HtmlItemExpr),
}

pub(crate) struct HtmlItemExpr {
    pub paren: token::Paren,
    pub value: syn::Expr,
}

impl ToTokens for HtmlItemExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren.surround(tokens, |tokens| {
            self.value.to_tokens(tokens);
        });
    }
}

impl Parse for HtmlItemExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: syn::parenthesized!(content in input),
            value: content.parse()?
        })
    }
}