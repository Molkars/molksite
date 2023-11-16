use std::borrow::Cow;
use dparse::{punct, ident, keywords};
use dparse::parse::{Parse, ParseError, ParseStream};
use dparse_derive::Parse;
use crate::hscript::program::Program;
use crate::html::Tag;

mod litstr;

mod program;
mod expr;
mod parselt;


#[test]
fn simple() {
    let input = r#"
#include "nav.html"

<div id="root">
    <h1>Hello, world!</h1>
    <h2>Dillon Shaffer</h2>
</div>
    "#;

    let mut stream = ParseStream::new(input);
    let program = Program::parse(&mut stream).unwrap();
    println!("{:#?}", program);
}