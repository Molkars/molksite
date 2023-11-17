mod program;
mod expr;


#[cfg(test)]
mod tests {
    use dparse::parse::{Parse, ParseStream};
    use crate::hscript::program::Program;

    #[test]
    fn simple() {
        let input = r#"
#include "nav.html"

#if mode == "dev"
    <div>DEV MODE</div>
#end ""

<div id="root">
    <h1>Hello, world!</h1>
    <h2>Dillon Shaffer</h2>
</div>
    "#;

        let mut stream = ParseStream::new(input);
        let program = Program::parse(&mut stream)
            .map_err(|e| {
                eprintln!("{}", e);
                e
            })
            .unwrap();
        println!("{:#?}", program);
    }
}