use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub mod prelude {
    pub use super::{Tag, Child};

    #[macro_export]
    macro_rules! tag {
        ($(fn $n:ident;)+) => {
            $(
                #[allow(unused)]
                pub fn $n() -> Tag {
                    Tag::new(stringify!($n))
                }
            )*
        };
    }

    #[macro_export]
    macro_rules! text {
        ($(fn $n:ident;)*) => {
            $(
                #[allow(unused)]
                pub fn $n(c: impl Into<Child>) -> Tag {
                    Tag::new(stringify!($n))
                        .child(c)
                }
            )*
        };
    }

    tag! {
        fn div;
        fn a;
        fn img;
        fn ul;
        fn li;
        fn ol;
        fn table;
        fn tr;
        fn td;
        fn th;
        fn tbody;
        fn thead;
        fn tfoot;
        fn form;
        fn input;
        fn button;
        fn textarea;
        fn select;
        fn header;
        fn footer;
        fn main;
        fn body;
    }

    text! {
        fn h1;
        fn h2;
        fn h3;
        fn h4;
        fn h5;
        fn h6;
        fn p;
        fn pre;
        fn span;
        fn strong;
        fn em;
        fn code;
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Child {
    Tag(Tag),
    Text(Cow<'static, str>),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Tag {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Child>,
    closing: Closing,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Closing {
    Inline,
    Implicit,
    Explicit,
}

impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        format!("{}", value)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;
        for attr in &self.attributes {
            match &attr.value {
                Some(value) => write!(f, " {}=\"{}\"", attr.name, value)?,
                None => write!(f, " {}", attr.name)?,
            }
        }
        write!(f, ">")?;
        for child in &self.children {
            match child {
                Child::Tag(tag) => write!(f, "{}", tag)?,
                Child::Text(text) => write!(f, "{}", text)?,
            }
        }
        write!(f, "</{}>", self.name)?;
        Ok(())
    }
}

impl From<Tag> for Child {
    fn from(value: Tag) -> Self {
        Child::Tag(value)
    }
}

impl From<&'static str> for Child {
    fn from(value: &'static str) -> Self {
        Child::Text(Cow::Borrowed(value))
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Attribute {
    name: String,
    value: Option<String>,
}

impl Tag {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            closing: Closing::Explicit,
        }
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push(Attribute {
            name: name.into(),
            value: Some(value.into()),
        });
        self
    }

    pub fn child(mut self, child: impl Into<Child>) -> Self {
        self.children.push(child.into());
        self
    }
}

mod parse {
    use std::borrow::Cow;
    use dparse::{basic, punct};
    use super::{Tag, Attribute, Closing, Child};
    use dparse::parse::{Parse, WithMessage, ParseError, ParseStream, Span};

    struct OpenTag {
        name: String,
        attributes: Vec<Attribute>,
        closed: bool,
    }

    struct ClosingTag {
        name: String,
    }

    punct! {
        pub struct LeftArrowSlash("</");
    }

    impl Parse<'_> for ClosingTag {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            if !input.did_parse::<LeftArrowSlash>() {
                return Err(input.mismatch());
            }
            let name = input.parse::<HtmlIdent>()
                .with_message("expected tag-name")?;
            input.require::<basic::RightArrow>()
                .with_message("expected '>'")?;
            Ok(Self {
                name: name.name,
            })
        }
    }

    impl Parse<'_> for Attribute {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            let Some(name) = input.try_parse::<HtmlIdent>()? else {
                return Err(input.mismatch());
            };

            let value = if input.try_parse::<basic::Equals>()?.is_some() {
                let value = input.parse::<HtmlStrLit>()
                    .with_message("expected string literal")?;
                Some(value.content)
            } else {
                None
            };

            Ok(Self {
                name: name.name,
                value,
            })
        }
    }

    impl Parse<'_> for Tag {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            if !input.did_parse::<basic::LeftArrow>() {
                return Err(input.mismatch());
            }
            let name = input.parse::<HtmlIdent>()
                .with_message("expected tag-name")?;

            let mut tag = Tag::new(name.name);
            while input.has_next() && input.peek_parse::<basic::RightArrow>().is_err() {
                let attr = input.parse::<Attribute>()
                    .with_message("expected attribute")?;
                tag.attributes.push(attr);
            }

            let inline_close = input.try_parse::<basic::Slash>()?
                .is_some();

            input.require::<basic::RightArrow>()
                .with_message("expected '>'")?;

            if inline_close {
                tag.closing = Closing::Inline;
            } else {
                while input.has_next() {
                    if let Some(closing_tag) = input.try_parse::<ClosingTag>()? {
                        let implicit_close = !closing_tag.name.eq_ignore_ascii_case(&tag.name);
                        if implicit_close {
                            tag.closing = Closing::Implicit;
                        } else {
                            tag.closing = Closing::Explicit;
                        }
                        break;
                    }

                    let child = input.parse::<Child>()
                        .with_message("expected child")?;
                    tag.children.push(child);
                }
            }

            Ok(tag)
        }
    }

    impl Parse<'_> for Child {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            if let Some(tag) = input.try_parse::<Tag>()? {
                Ok(Child::Tag(tag))
            } else {
                let text = input.take_while(|c| c != '<')
                    .to_string();
                Ok(Child::Text(Cow::Owned(text)))
            }
        }
    }


    pub struct HtmlIdent {
        name: String,
        span: Span,
    }

    impl Parse<'_> for HtmlIdent {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            input.take_while(|c| c.is_whitespace());

            if input.peek_char().filter(|c| c.is_ascii_alphanumeric()).is_none() {
                return Err(input.mismatch());
            }

            let spanner = input.spanner();
            let content = input.take_while(|c| c.is_ascii_alphanumeric());
            Ok(Self {
                name: content.to_string(),
                span: input.span(spanner),
            })
        }
    }

    pub struct HtmlStrLit {
        content: String,
        span: Span,
    }

    impl Parse<'_> for HtmlStrLit {
        fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
            input.take_while(|c| c.is_whitespace());

            if !input.did_parse::<basic::Quote>() {
                return Err(input.mismatch());
            }

            let spanner = input.spanner();
            let content = input.take_while(|c| c != '"');
            input.require::<basic::Quote>()
                .with_message("expected '\"'")?;

            Ok(Self {
                content: content.to_string(),
                span: input.span(spanner),
            })
        }
    }

    #[test]
    fn test() {
        let input = r#"<div class="test" id="test">Hello, world!</div>"#;
        let mut stream = ParseStream::new(input);
        let tag = stream.parse::<Tag>().unwrap();
        println!("{}", tag);
        assert!(!stream.has_next());
        assert_eq!(tag.name, "div");
        assert_eq!(tag.attributes.len(), 2);
        assert_eq!(tag.children.len(), 1);
        assert_eq!(tag.closing, Closing::Explicit);

        println!("{:?}", tag);
    }
}