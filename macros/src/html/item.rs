use proc_macro2::TokenStream;
use syn::parse::ParseStream;
use syn::{token, Lifetime, Pat, Token};
use crate::html;

pub(crate) enum Item {
    Element(html::Element),
    Lit(syn::Lit),
    Embedded(Embedded),
    For(ExprForLoop),
    If(ExprIf),
    Match(Match),
    Local(Local),
    Block(Block),
    Return(ExprReturn),
    Break(ExprBreak),
    Continue(ExprContinue),
    Yield(ExprYield),
}

pub(crate) struct Embedded {
    pub paren: token::Paren,
    pub value: syn::Expr,
}

pub(crate) struct ExprForLoop {
    pub token: Token![for],
    pub pat: Pat,
    pub in_token: Token![in],
    pub expr: syn::Expr,
    pub block: Block,
}

pub(crate) struct Block {
    pub brace: token::Brace,
    pub content: Vec<Item>,
}

pub(crate) struct ExprIf {
    pub if_token: Token![if],
    pub cond: syn::Expr,
    pub then_branch: Block,
    pub else_branch: Option<(Token![else], Box<ExprElse>)>,
}

pub(crate) enum ExprElse {
    If(ExprIf),
    Block(Block),
}

pub(crate) struct Match {
    pub match_token: Token![match],
    pub expr: syn::Expr,
    pub brace_token: token::Brace,
    pub arms: Vec<Arm>,
}

pub(crate) struct Arm {
    pub pat: Pat,
    pub guard: Option<(Token![if], syn::Expr)>,
    pub fat_arrow_token: Token![=>],
    pub body: Item,
    pub comma: Option<Token![,]>,
}

pub(crate) struct Local {
    pub let_token: Token![let],
    pub pat: Pat,
    pub init: Option<LocalInit>,
    pub semi_token: Token![;],
}

pub(crate) struct LocalInit {
    pub eq_token: Token![=],
    pub expr: syn::Expr,
    pub diverge: Option<(Token![else], Block)>,
}

pub(crate) struct ExprReturn {
    pub return_token: Token![return],
    pub expr: Option<Box<Item>>,
}

pub(crate) struct ExprBreak {
    pub break_token: Token![break],
    pub label: Option<Lifetime>,
    pub expr: Option<Box<Item>>,
}

pub(crate) struct ExprContinue {
    pub continue_token: Token![continue],
    pub label: Option<Lifetime>,
}

pub(crate) struct ExprYield {
    pub yield_token: Token![yield],
    pub expr: Option<Box<Item>>,
}

mod to_tokens {
    use super::*;
    use quote::{ToTokens, TokenStreamExt};

    impl ToTokens for Item {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Item::Element(item) => item.to_tokens(tokens),
                Item::Lit(item) => item.to_tokens(tokens),
                Item::Embedded(item) => item.to_tokens(tokens),
                Item::For(item) => item.to_tokens(tokens),
                Item::If(item) => item.to_tokens(tokens),
                Item::Match(item) => item.to_tokens(tokens),
                Item::Local(item) => item.to_tokens(tokens),
                Item::Block(item) => item.to_tokens(tokens),
                Item::Return(item) => item.to_tokens(tokens),
                Item::Break(item) => item.to_tokens(tokens),
                Item::Continue(item) => item.to_tokens(tokens),
                Item::Yield(item) => item.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for Embedded {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.paren.surround(tokens, |tokens| {
                self.value.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for ExprForLoop {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
            self.in_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.block.to_tokens(tokens);
        }
    }

    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace.surround(tokens, |tokens| {
                tokens.append_all(&self.content);
            });
        }
    }

    impl ToTokens for ExprIf {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.if_token.to_tokens(tokens);
            self.cond.to_tokens(tokens);
            self.then_branch.to_tokens(tokens);
            if let Some((kw, branch)) = &self.else_branch {
                kw.to_tokens(tokens);
                branch.to_tokens(tokens);
            }
        }
    }

    impl ToTokens for ExprElse {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                ExprElse::If(item) => item.to_tokens(tokens),
                ExprElse::Block(item) => item.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for Match {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.match_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(&self.arms);
            });
        }
    }

    impl ToTokens for Arm {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.pat.to_tokens(tokens);
            if let Some((kw, expr)) = &self.guard {
                kw.to_tokens(tokens);
                expr.to_tokens(tokens);
            }
            self.fat_arrow_token.to_tokens(tokens);
            self.body.to_tokens(tokens);
            self.comma.to_tokens(tokens);
        }
    }

    impl ToTokens for Local {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.let_token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
            self.init.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for LocalInit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            if let Some((kw, expr)) = &self.diverge {
                kw.to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }

    impl ToTokens for ExprReturn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.return_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprBreak {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.break_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprContinue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.continue_token.to_tokens(tokens);
            self.label.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprYield {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.yield_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }
}

mod parsing {
    use syn::Error;
    use syn::parse::discouraged::Speculative;
    use super::*;
    use syn::parse::Parse;

    impl Parse for Item {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![<]) {
                input.parse().map(Self::Element)
            } else if input.peek(syn::Lit) {
                input.parse().map(Self::Lit)
            } else if input.peek(token::Paren) {
                input.parse().map(Self::Embedded)
            } else if input.peek(Token![for]) {
                input.parse().map(Self::For)
            } else if input.peek(Token![if]) {
                input.parse().map(Self::If)
            } else if input.peek(Token![let]) {
                input.parse().map(Self::Local)
            } else if input.peek(syn::token::Brace) {
                input.parse().map(Self::Block)
            } else if input.peek(Token![return]) {
                input.parse().map(Self::Return)
            } else if input.peek(Token![break]) {
                input.parse().map(Self::Break)
            } else if input.peek(Token![continue]) {
                input.parse().map(Self::Continue)
            } else if input.peek(Token![yield]) {
                input.parse().map(Self::Yield)
            } else {
                Err(input.error("expected element, (expr), or control element"))
            }
        }
    }

    impl Parse for Embedded {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                paren: syn::parenthesized!(content in input),
                value: content.parse()?,
            })
        }
    }

    impl Parse for ExprForLoop {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                token: input.parse()?,
                pat: Pat::parse_multi_with_leading_vert(input)?,
                in_token: input.parse()?,
                expr: input.call(syn::Expr::parse_without_eager_brace)?,
                block: input.parse()?,
            })
        }
    }

    impl Parse for Block {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                brace: syn::braced!(content in input),
                content: content.call(|input| {
                    let mut out = Vec::new();
                    while !input.is_empty() {
                        out.push(input.parse()?);
                    }
                    Ok(out)
                })?,
            })
        }
    }


    impl Parse for ExprIf {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                if_token: input.parse()?,
                cond: input.parse()?,
                then_branch: input.parse()?,
                else_branch: if input.peek(Token![else]) {
                    Some((input.parse()?, input.parse()?))
                } else {
                    None
                },
            })
        }
    }

    impl Parse for ExprElse {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![if]) {
                input.parse().map(Self::If)
            } else {
                input.parse().map(Self::Block)
            }
        }
    }

    impl Parse for Match {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                match_token: input.parse()?,
                expr: input.parse()?,
                brace_token: syn::braced!(content in input),
                arms: content.call(|input| {
                    let mut out = Vec::new();
                    while !input.is_empty() {
                        out.push(input.parse()?);
                    }
                    Ok(out)
                })?,
            })
        }
    }
    pub(crate) fn requires_comma_to_be_match_arm(expr: &syn::Expr) -> bool {
        match expr {
            syn::Expr::If(_)
            | syn::Expr::Match(_)
            | syn::Expr::Block(_) | syn::Expr::Unsafe(_) // both under ExprKind::Block in rustc
            | syn::Expr::While(_)
            | syn::Expr::Loop(_)
            | syn::Expr::ForLoop(_)
            | syn::Expr::TryBlock(_)
            | syn::Expr::Const(_) => false,

            syn::Expr::Array(_)
            | syn::Expr::Assign(_)
            | syn::Expr::Async(_)
            | syn::Expr::Await(_)
            | syn::Expr::Binary(_)
            | syn::Expr::Break(_)
            | syn::Expr::Call(_)
            | syn::Expr::Cast(_)
            | syn::Expr::Closure(_)
            | syn::Expr::Continue(_)
            | syn::Expr::Field(_)
            | syn::Expr::Group(_)
            | syn::Expr::Index(_)
            | syn::Expr::Infer(_)
            | syn::Expr::Let(_)
            | syn::Expr::Lit(_)
            | syn::Expr::Macro(_)
            | syn::Expr::MethodCall(_)
            | syn::Expr::Paren(_)
            | syn::Expr::Path(_)
            | syn::Expr::Range(_)
            | syn::Expr::Reference(_)
            | syn::Expr::Repeat(_)
            | syn::Expr::Return(_)
            | syn::Expr::Struct(_)
            | syn::Expr::Try(_)
            | syn::Expr::Tuple(_)
            | syn::Expr::Unary(_)
            | syn::Expr::Yield(_)
            | syn::Expr::Verbatim(_) => true,
            _ => unimplemented!("rust was updated!"),
        }
    }

    impl Parse for Arm {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                pat: Pat::parse_multi_with_leading_vert(input)?,
                guard: {
                    if input.peek(Token![if]) {
                        let if_token: Token![if] = input.parse()?;
                        let guard = input.parse()?;
                        Some((if_token, guard))
                    } else {
                        None
                    }
                },
                fat_arrow_token: input.parse()?,
                body: input.parse()?,
                comma: input.parse()?,
            })
        }
    }

    impl Parse for Local {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                let_token: input.parse()?,
                pat: {
                    let mut pat = Pat::parse_single(input)?;
                    if input.peek(Token![:]) {
                        let colon_token: Token![:] = input.parse()?;
                        let ty: syn::Type = input.parse()?;
                        pat = Pat::Type(syn::PatType {
                            attrs: Vec::new(),
                            pat: Box::new(pat),
                            colon_token,
                            ty: Box::new(ty),
                        });
                    }
                    pat
                },
                init: {
                    if let Some(eq_token) = input.parse()? {
                        let eq_token: Token![=] = eq_token;
                        let expr = input.parse()?;

                        let diverge = if input.peek(Token![else]) {
                            let else_token: Token![else] = input.parse()?;
                            let diverge = input.parse()?;
                            Some((else_token, diverge))
                        } else {
                            None
                        };

                        Some(LocalInit {
                            eq_token,
                            expr,
                            diverge,
                        })
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for LocalInit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                eq_token: input.parse()?,
                expr: input.parse()?,
                diverge: if let Ok(token) = input.parse() {
                    Some((token, input.parse()?))
                } else {
                    None
                },
            })
        }
    }

    fn can_begin_expr(input: ParseStream) -> bool {
        input.peek(syn::Ident) // value name or keyword
            || input.peek(token::Paren) // tuple
            || input.peek(token::Bracket) // array
            || input.peek(token::Brace) // block
            || input.peek(syn::Lit) // literal
            || input.peek(Token![!]) && !input.peek(Token![!=]) // operator not
            || input.peek(Token![-]) && !input.peek(Token![-=]) && !input.peek(Token![->]) // unary minus
            || input.peek(Token![*]) && !input.peek(Token![*=]) // dereference
            || input.peek(Token![|]) && !input.peek(Token![|=]) // closure
            || input.peek(Token![&]) && !input.peek(Token![&=]) // reference
            || input.peek(Token![..]) // range notation
            || input.peek(Token![<]) && !input.peek(Token![<=]) && !input.peek(Token![<<=]) // associated path
            || input.peek(Token![::]) // global path
            || input.peek(syn::Lifetime) // labeled loop
            || input.peek(Token![#]) // expression attributes
    }

    impl Parse for ExprReturn {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(ExprReturn {
                return_token: input.parse()?,
                expr: {
                    if can_begin_expr(input) {
                        Some(input.parse()?)
                    } else {
                        None
                    }
                },
            })
        }
    }

    impl Parse for ExprBreak {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let break_token: Token![break] = input.parse()?;

            let ahead = input.fork();
            let label: Option<syn::Lifetime> = ahead.parse()?;
            if label.is_some() && ahead.peek(Token![:]) {
                // Not allowed: `break 'label: loop {...}`
                // Parentheses are required. `break ('label: loop {...})`
                let _: syn::Expr = input.parse()?;
                let start_span = label.unwrap().apostrophe;
                return Err(Error::new(
                    start_span.join(input.span()).unwrap(),
                    "parenthesis required",
                ));
            }

            input.advance_to(&ahead);
            let expr = if can_begin_expr(input) && !input.peek(token::Brace) {
                Some(input.parse()?)
            } else {
                None
            };

            Ok(ExprBreak {
                break_token,
                label,
                expr,
            })
        }
    }

    impl Parse for ExprContinue {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                continue_token: input.parse()?,
                label: input.parse()?,
            })
        }
    }

    impl Parse for ExprYield {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(ExprYield {
                yield_token: input.parse()?,
                expr: {
                    if can_begin_expr(input) {
                        Some(input.parse()?)
                    } else {
                        None
                    }
                },
            })
        }
    }
}