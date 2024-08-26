use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use macros::html;
use crate::routes::AppState;
use crate::ui::{icons, PageBuilder};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
}

async fn index() -> Response {
    PageBuilder::default()
        .title("C2 | molkars.dev")
        .body(html! {
            <div class="flex flex-col h-screen">
            <header class="bg-slate-200">
                <div class="stack">
                <div class="px-4 sm:px-6 lg:px-8 max-w-screen-xl">
                    <div class="flex h-16 items-center justify-between">
                        (icons::Button::new("Menu", icons::Menu::default()))
                        <div class="md:flex md:items-center md:gap-12">
                            <a class="block text-teal-600" href="/">
                              <span class="sr-only">"molkars.dev"</span>
                              <p>"molkars.dev"</p>
                            </a>
                        </div>
                    </div>
                </div>
                </div>
            </header>
            <div class="flex-1 grid grid-cols-24 max-w-screen">
                <section id="drawer" class="col-span-4 bg-slate-100">
                </section>
            </div>
            </div>
        })
        .compile()
        .into_response()
}