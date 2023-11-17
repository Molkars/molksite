use dparse::basic;
use dparse_derive::Parse;
use crate::hscript::expr::Expr;
use crate::html::Tag;

#[derive(Parse, Debug)]
pub struct Program {
    decls: Vec<Decl>,
}

#[derive(Parse, Debug)]
pub enum Decl {
    Include(Include),
    Tag(Tag),
}

#[derive(Parse, Debug)]
pub struct Include {
    pub pound: basic::Hash,
    pub name: basic::CIdent,
    pub path: Expr,
}
