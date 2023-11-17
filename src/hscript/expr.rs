use dparse::basic;
use dparse_derive::Parse;

#[derive(Parse, Debug)]
#[dparse(kind = "expr")]
pub enum Expr {
    Equality(Box<Self>, EqualityOp, Box<Self>),
    Term(Box<Self>, TermOp, Box<Self>),
    Factor(Box<Self>, FactorOp, Box<Self>),
    Unary(UnaryOp, Box<Self>),
    Primary(Primary),
}

#[derive(Parse, Debug)]
pub enum Primary {
    Ident(basic::CIdent),
    StrLit(basic::LitCStr),
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

#[derive(Parse, Debug)]
pub enum EqualityOp {
    Eq(basic::DoubleEquals),
    Neq(basic::BangEquals),
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