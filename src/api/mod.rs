mod infra;

pub use infra::{App, Api};
use about::About;
use index::Index;

mod index;
mod about;

pub fn app() -> App {
    App::default()
        .route("/", Index)
        .route("/about", About)
}
