use dparse_derive::Parse;
use crate::hscript::litstr;
use crate::hscript::parselt::{Ident, Pound};
use crate::html::Tag;

#[derive(Parse, Debug)]
pub struct Program {
    decls: Vec<Decl>,
}

#[derive(Parse, Debug)]
pub enum Decl {
    Command(Command),
    Tag(Tag),
}

#[derive(Parse, Debug)]
pub struct Command {
    pub pound: Pound,
    pub name: Ident,
    pub path: litstr::StrLit,
}
