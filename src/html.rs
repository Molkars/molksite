#![allow(dead_code)]

use std::borrow::Cow;

pub const DOCTYPE: RawHtml = RawHtml::from_static("<!DOCTYPE html>\n");

#[derive(Default)]
pub struct HtmlBundle(Vec<Child>);

impl HtmlBundle {
    pub fn add_element(&mut self, element: Element) {
        self.0.push(Child::Element(element));
    }

    pub fn add_text(&mut self, text: impl Render) {
        self.0.push(Child::Content(text.render()));
    }
}

pub enum ElementStyle {
    Normal,
    Unclosed,
    Inline,
}

pub struct Element {
    tag_name: String,
    element_style: ElementStyle,
    attrs: Vec<(String, Option<String>)>,
    children: HtmlBundle,
}

impl Element {
    pub fn new(tag_name: impl Render, element_style: ElementStyle) -> Self {
        Self {
            tag_name: tag_name.render(),
            element_style,
            attrs: Vec::new(),
            children: HtmlBundle(Vec::new()),
        }
    }

    pub fn attr_opt(mut self, key: impl Render, value: Option<impl Render>) -> Self {
        self.attrs.push((key.render(), value.as_ref().map(Render::render)));
        self
    }

    pub fn with_children(mut self, f: impl FnOnce(&mut HtmlBundle)) -> Self {
        f(&mut self.children);
        self
    }
}

enum Child {
    Content(String),
    Element(Element),
}

pub struct RawHtml(Cow<'static, str>);

impl RawHtml {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self(content.into())
    }

    pub const fn from_static(content: &'static str) -> Self {
        Self(Cow::Borrowed(content))
    }
}

pub trait Render {
    fn render_to<W: std::fmt::Write>(&self, out: &mut W) -> Result<(), Box<dyn std::error::Error>>;

    #[inline]
    fn render(&self) -> String {
        self.try_render().unwrap()
    }

    fn try_render(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut out = String::new();
        self.render_to(&mut out)?;
        Ok(out)
    }
}

mod rendering {
    use std::error::Error;
    use std::fmt::Write;
    use crate::html::{Child, Element, ElementStyle, RawHtml, Render};

    use html_escape::{encode_quoted_attribute, encode_safe, encode_text};

    impl Render for super::HtmlBundle {
        fn render_to<W: Write>(&self, f: &mut W) -> Result<(), Box<dyn Error>> {
            for child in &self.0 {
                child.render_to(f)?;
            }
            Ok(())
        }
    }

    impl Render for Element {
        fn render_to<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn Error>> {
            write!(out, "<{}", encode_safe(&self.tag_name))?;
            for (name, value) in &self.attrs {
                match value {
                    Some(v) => write!(out, " {}=\"{}\"", encode_safe(&name), encode_quoted_attribute(&v)),
                    None => write!(out, " {}", encode_safe(&name))
                }?;
            }
            match self.element_style {
                ElementStyle::Normal => {
                    writeln!(out, ">")?;
                    self.children.render_to(out)?;
                    writeln!(out, "</{}>", encode_safe(&self.tag_name))?;
                }
                ElementStyle::Unclosed => writeln!(out, ">")?,
                ElementStyle::Inline => writeln!(out, "/>")?,
            };

            Ok(())
        }
    }

    impl Render for Child {
        fn render_to<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn Error>> {
            match self {
                Child::Content(value) => write!(out, "{}", value)?,
                Child::Element(e) => e.render_to(out)?,
            };
            Ok(())
        }
    }

    impl Render for RawHtml {
        fn render_to<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn Error>> {
            write!(out, "{}", self.0)?;
            Ok(())
        }
    }

    macro_rules! render_to_string_impl {
        ($t:ty) => {
            impl Render for $t {
                fn render_to<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn Error>> {
                    write!(out, "{}", encode_text(&self.to_string()))?;
                    Ok(())
                }
            }
        };
    }

    render_to_string_impl!(i8);
    render_to_string_impl!(i16);
    render_to_string_impl!(i32);
    render_to_string_impl!(i64);
    render_to_string_impl!(i128);
    render_to_string_impl!(isize);
    render_to_string_impl!(u8);
    render_to_string_impl!(u16);
    render_to_string_impl!(u32);
    render_to_string_impl!(u64);
    render_to_string_impl!(u128);
    render_to_string_impl!(usize);
    render_to_string_impl!(f32);
    render_to_string_impl!(f64);
    render_to_string_impl!(bool);
    render_to_string_impl!(&'_ str);
    render_to_string_impl!(String);
}

mod axum {
    use axum::response::{Html, IntoResponse, Response};
    use axum::http::{HeaderValue, StatusCode};
    use axum::body::Body;
    use axum::http::header::CONTENT_TYPE;
    use crate::html::{HtmlBundle, Render};

    impl IntoResponse for HtmlBundle {
        fn into_response(self) -> Response {
            match self.try_render() {
                Ok(content) => {

                    let mut response = Response::new(Body::new(content));
                    *response.status_mut() = StatusCode::OK;
                    response.headers_mut()
                        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
                    response
                },
                Err(e) => {
                    eprintln!("error rendering html: {e}");
                    let res = Html(r#"
<html>
    <head>
        <title>oops!</title>
    </head>
    <body>
    <div style="display: flex; justify-content: center; align-items: center; font-size: 44px">
        Uh Oh! An error occurred!
    </div>
    </body>
</html>
"#);
                    (StatusCode::INTERNAL_SERVER_ERROR, res).into_response()
                }
            }
        }
    }
}