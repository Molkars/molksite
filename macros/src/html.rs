use proc_macro2::TokenStream;
use syn::Error;

mod ident;
pub(crate) use ident::*;

mod name;
pub(crate) use name::*;

mod item;
pub(crate) use item::*;

mod opening_tag;
pub(crate) use opening_tag::*;

mod closing_tag;
pub(crate) use closing_tag::*;

mod element;
pub(crate) use element::*;

mod attribute;
pub(crate) use attribute::*;

pub fn html(input: TokenStream) -> Result<TokenStream, Error> {
    let out = syn::parse2::<Html>(input)?;
    Ok(out.generate())
}

