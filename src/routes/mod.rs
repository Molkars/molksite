use axum::Router;
use axum::routing::get;
use macros::html;
use crate::html::HtmlBundle;
use crate::ui::{PageBuilder};

pub(super) fn routes() -> Router {
    Router::new()
        .route("/", get(index))
}

async fn index() -> HtmlBundle {
    PageBuilder::default()
        .title("Molkars | Dillon Shaffer")
        .body(html! {
            <main>
                <header>
                    <h1>("Hello!")</h1>
                </header>
            </main>
        })
        .build()
        .unwrap()
        .into()
}