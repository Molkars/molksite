use std::borrow::Cow;
use dparse::parse::{Parse, ParseError, ParseStream};
use dparse::punct;

punct! {
    pub struct Pound("#");
}

#[derive(Debug)]
pub struct Ident {
    value: Cow<'static, str>,
}

impl Ident {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Parse<'_> for Ident {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        if input.peek_char().filter(|c| c.is_alphabetic()).is_none() {
            return Err(input.mismatch());
        }
        let content = input.take_while(|c| {
            c.is_alphanumeric() || c == '_' || c == '-'
        }).to_string();
        Ok(Self {
            value: content.into(),
        })
    }
}
