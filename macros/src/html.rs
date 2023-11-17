use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, LitStr};
use syn::parse::{Parse, ParseStream};
use syn::parse::discouraged::Speculative;

/*
html! {
    struct UserView(
        name: String
    ) {
        <div>
            <p>hello
            <strong>$name</strong>,
            how are you?
        </div>
    }
}
 */

pub fn html(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let item = syn::parse2::<Input>(input)
        .map_err(|e| {
            let mut tokens = TokenStream::new();
            e.to_compile_error().to_tokens(&mut tokens);
            tokens
        })?;

    let Input { attrs, vis, name, body } = item;
    let body = body.to_string();
    let body = LitStr::new(&body, Span::call_site());
    Ok(quote! {
        #(#attrs)*
        #vis struct #name;

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #body)
            }
        }
    })
}

struct Input {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    name: syn::Ident,
    body: Tag,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        let vis = input.parse::<syn::Visibility>()?;
        input.parse::<syn::Token![struct]>()?;
        let name = input.parse::<syn::Ident>()?;

        let content;
        syn::braced!(content in input);
        let body = content.parse::<Tag>()?;
        Ok(Self {
            attrs,
            vis,
            name,
            body,
        })
    }
}

struct Tag {
    name: syn::Ident,
    attrs: BTreeMap<syn::Ident, HtmlAttr>,
    children: Vec<Child>,
    closing: ClosingTag,
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        let name = input.parse::<syn::Ident>()?;

        let mut attrs = BTreeMap::new();
        while input.peek(syn::Ident) {
            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![=]>()?;
            let value = input.parse::<syn::Expr>()?;
            attrs.insert(name.clone(), HtmlAttr { name, value });
        }

        if input.parse::<InlineCloseTag>().is_ok() {
            Ok(Self {
                name,
                attrs,
                children: Vec::new(),
                closing: ClosingTag::Inline,
            })
        } else {
            input.parse::<syn::Token![>]>()?;

            let mut children = Vec::new();
            let mut closing = ClosingTag::Implicit;

            loop {
                let input2 = input.fork();
                if input2.parse::<ExplicitCloseTag>().is_ok() {
                    let tag_name = input2.parse::<syn::Ident>()
                        .map_err(|e| Error::new(e.span(), "expected closing tag name"))?;
                    input2.parse::<syn::Token![>]>()
                        .map_err(|e| Error::new(e.span(), "expected `>`"))?;
                    if name == tag_name {
                        closing = ClosingTag::Explicit(tag_name);
                        input.advance_to(&input2);
                    }
                    break;
                }

                if input.peek(syn::Token![<]) {
                    let child = input.parse::<Tag>()
                        .map_err(|e| Error::new(e.span(), format!("expected tag: {}", e)))?;
                    children.push(Child::Tag(child));
                } else {
                    let text = input.parse::<syn::LitStr>()
                        .map_err(|e| Error::new(e.span(), "expected string"))?;
                    children.push(Child::Text(text.value()));
                }
            }

            Ok(Self {
                name,
                attrs,
                children,
                closing,
            })
        }
    }
}

syn::custom_punctuation!(ExplicitCloseTag, </);
syn::custom_punctuation!(InlineCloseTag, />);
enum ClosingTag {
    Inline,
    Explicit(syn::Ident),
    Implicit,
}

struct HtmlAttr {
    name: syn::Ident,
    value: syn::Expr,
}

enum Child {
    Text(String),
    Tag(Tag),
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;

        for HtmlAttr { name, value } in self.attrs.values() {
            write!(f, " {}=\"{}\"", name, value.to_token_stream().to_string())?;
        }

        match self.closing {
            ClosingTag::Inline => write!(f, "/>")?,
            ClosingTag::Explicit(ref name) => {
                write!(f, ">")?;
                for child in &self.children {
                    write!(f, "{}", child)?;
                }
                write!(f, "</{}>", name)?;
            }
            ClosingTag::Implicit => {
                write!(f, ">")?;
                for child in &self.children {
                    write!(f, "{}", child)?;
                }
            }
        };

        Ok(())
    }
}

impl Display for Child {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Child::Text(text) => write!(f, "{}", text)?,
            Child::Tag(tag) => write!(f, "{}", tag)?,
        }
        Ok(())
    }
}