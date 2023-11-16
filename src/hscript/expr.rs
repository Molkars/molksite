use dparse::basic;
use dparse::parse::{Parse};
use dparse_derive::Parse;
use crate::hscript::litstr::StrLit;
use crate::hscript::parselt::Ident;


#[derive(Parse, Debug)]
#[dparse(kind = "expr")]
pub enum Expr {
    Term(Box<Self>, TermOp, Box<Self>),
    Factor(Box<Self>, FactorOp, Box<Self>),
    Unary(UnaryOp, Box<Self>),
    Primary(Primary),
}

#[derive(Parse, Debug)]
pub enum Primary {
    Ident(Ident),
    StrLit(StrLit),
    // LitInt(basic::LitInt),
}

#[derive(Parse, Debug)]
pub enum UnaryOp {
    Not(basic::Bang),
    Neg(basic::Dash),
}

#[derive(Parse, Debug)]
pub enum TermOp {
    Mul(basic::Star),
    Div(basic::Slash),
    Rem(basic::Percent),
}

#[derive(Parse, Debug)]
pub enum FactorOp {
    Add(basic::Plus),
    Sub(basic::Dash),
}

#[cfg(test)]
mod test {
    use dparse::parse::{Parse, ParseStream};
    use crate::hscript::expr::Expr;

    #[test]
    fn test_simple() {
        let raw = "a + b * c";
        let mut stream = ParseStream::new(raw);
        let expr = Expr::parse(&mut stream).unwrap();
        println!("{:#?}", expr);
    }
}