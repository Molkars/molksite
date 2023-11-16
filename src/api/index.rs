use crate::api::Api;
use crate::api::infra::Response;
use crate::html::prelude::{body, div, h1, h2, header, li, main, p, ul};

pub struct Index;

impl Api for Index {
    fn get(&self) -> Response {
        Response::html(
            body()
                .child(header()
                    .child(p("Welcome to the Rusty Web Server!")))
                .child(main()
                    .child(h1("Index"))
                    .child(p("This is the index page.")))
        )
    }
}