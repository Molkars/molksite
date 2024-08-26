use derive_builder::Builder;
use macros::html;
use crate::html::*;

pub mod components;
pub mod icons;

pub fn head(title: impl Into<String>) -> HtmlBundle {
    html! {
        <head>
            <title>(title.into())</title>
            <script
                src="https://unpkg.com/htmx.org@2.0.2"
                integrity="sha384-Y7hw+L/jvKeWIRRkqWYfPcvVxHzVzn5REgzbawhxAuQGwX1XWe70vji+VSeHOThJ"
                crossorigin="anonymous"></script>
            <link
                href="https://unpkg.com/normalize.css"
                rel="stylesheet"/>
            <link
                href="https://unpkg.com/sanitize.css"
                rel="stylesheet" />
            <link
                href="https://unpkg.com/sanitize.css/forms.css"
                rel="stylesheet"/>
            <link
                href="https://unpkg.com/sanitize.css/typography.css"
                rel="stylesheet" />
            <link href="static/css/custom.css" rel="stylesheet">
            <link href="static/css/global.css" rel="stylesheet">
            <link href="static/css/button.css" rel="stylesheet">
            <link href="static/css/header.css" rel="stylesheet">
            // <script src="https://cdn.jsdelivr.net/npm/@unocss/runtime"></script>
            // <script>(RawHtml::from_static(include_str!("unocss.js")))</script>
        </head>
    }
}

#[derive(Builder)]
#[builder(pattern="owned", setter(into))]
pub struct Page {
    title: String,
    #[builder(default)]
    body: HtmlBundle,
}

impl PageBuilder {
    pub fn compile(self) -> HtmlBundle {
        self.build().unwrap().into()
    }
}

impl Into<HtmlBundle> for Page {
    fn into(self) -> HtmlBundle {
        html! {
            (DOCTYPE)
            <html>
                (head(self.title))
                <body id="body">
                    (self.body)
                </body>
            </html>
        }
    }
}

macro_rules! page {
    (
        $title:expr;
        $($t:tt)*
    ) => {
        crate::ui::PageBuilder::default()
            .title($title)
            .body(macros::html! { $($t)* })
            .compile()
    };
}
pub(crate) use page;