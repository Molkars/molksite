use axum::Router;
use axum::routing::get;
use crate::html::{HtmlBundle};
use crate::ui::*;

pub(super) fn routes() -> Router {
    Router::new()
        .route("/", get(index))
}

async fn index() -> HtmlBundle {
    page! {
        "Home";

        <header>
            <div id="page-title">
                <h1>"molkars.dev"</h1>
            </div>
            <div class="flex-2"></div>
            <nav id="nav-items" class="flex-1">
                <a href="/dillon">"Dillon"</a>
            </nav>
            <div>
                <button hx-trigger="click">"Click Me!"</button>
            </div>
        </header>

        <section>
            <main>
                "Hello!"
            </main>
        </section>
    }
}