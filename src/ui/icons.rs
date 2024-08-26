use std::error::Error;
use std::fmt::Write;
use macros::html;
use crate::html::{HtmlBundle, Render};

mod menu;
pub use menu::Menu;

pub struct Button {
    label: String,
    content: HtmlBundle,
}

impl Button {
    pub fn new(label: impl Into<String>, value: impl Into<HtmlBundle>) -> Self {
        Self {
            label: label.into(),
            content: value.into(),
        }
    }
}

impl Render for Button {
    fn render_to<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn Error>> {
        let content = html! {
          <a
            class="inline-block
                p-2
                border focus:outline-none focus:ring
                text-current active:text-fuchsia-500 hover:text-fuchsia-600
                border-fuchsia-600"
            href="#"
          >
            <span class="sr-only"> (self.label) </span>
            (self.content)
          </a>
        };
        content.render_to(out)
    }
}