use derive_builder::Builder;
use macros::html;
use crate::html::*;

pub fn head(title: impl Into<String>) -> HtmlBundle {
    html! {
        <head>
            <title>(title.into())</title>
            <script
                src="https://unpkg.com/htmx.org@2.0.2"
                integrity="sha384-Y7hw+L/jvKeWIRRkqWYfPcvVxHzVzn5REgzbawhxAuQGwX1XWe70vji+VSeHOThJ"
                crossorigin="anonymous"></script>
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

impl Into<HtmlBundle> for Page {
    fn into(self) -> HtmlBundle {
        html! {
            (DOCTYPE)
            <html>
                (head(self.title))
                <body>
                    (self.body)
                </body>
            </html>
        }
    }
}