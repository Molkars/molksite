mod litstr;

mod program;
mod expr;
mod parselt;


#[cfg(test)]
mod tests {
    use dparse::parse::{Parse, ParseStream};
    use crate::hscript::program::Program;

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
}