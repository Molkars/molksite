use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use axum::extract::{FromRef, Path, State};
use axum::{Form, Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use keepcalm::SharedMut;
use macros::html;
use serde_derive::Deserialize;
use crate::html::{Compile, HtmlBundle};
use crate::ui::{PageBuilder};
use crate::ui::components::{Destination, DetailsListBuilder, Header, HeaderBuilder, HeaderButton, HeaderLogo};

mod expirmental_api;
mod c2;

type NotesRef = SharedMut<Notes>;

#[derive(Default)]
pub struct Notes {
    notes: HashMap<u64, NoteRef>,
    note_id: AtomicU64,
}

impl Notes {
    pub fn get_note(&self, id: u64) -> Option<NoteRef> {
        self.notes.get(&id).cloned()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u64, NoteRef> {
        self.notes.iter()
    }

    pub fn add_note(&mut self, mut note: Note) {
        let id = self.note_id.fetch_add(1, Ordering::Release);
        note.id = id;
        self.notes.insert(id, NoteRef::new(note));
    }
}

#[derive(Clone)]
pub struct AppState {
    pub notes: NotesRef,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            notes: NotesRef::new(Notes::default()),
        }
    }
}

impl FromRef<AppState> for NotesRef {
    fn from_ref(input: &AppState) -> Self {
        input.notes.clone()
    }
}

type NoteRef = SharedMut<Note>;

#[derive(Deserialize)]
pub struct Note {
    pub id: u64,
    pub name: String,
    pub description: String,
}

pub(super) fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/notes", get(notes_page))
        .route("/notes/new", get(new_note_page).post(add_note))
        .route("/notes/id/:id", get(note_overview_page))
        .nest("/c2", c2::routes())
        .with_state(AppState::default())
}

fn header_logo() -> HeaderLogo {
    HeaderLogo {
        href: "/".into(),
        label: "Home".into(),
        logo: html!( <p>"molkars.dev"</p> ),
    }
}

fn auth_buttons() -> Vec<HeaderButton> {
    vec![
        HeaderButton {
            content: html! {
              <a
                class="rounded-md bg-teal-600 px-5 py-2.5 text-sm font-medium text-white shadow"
                href="/auth/login"
              >
                "Login"
              </a>
            }
        },
        HeaderButton {
            content: html! {
              <div class="hidden sm:flex">
                <a
                  class="rounded-md bg-gray-100 px-5 py-2.5 text-sm font-medium text-teal-600"
                  href="/auth/register"
                >
                  "Register"
                </a>
              </div>
            },
        }
    ]
}

async fn index() -> HtmlBundle {
    PageBuilder::default()
        .title("Molkars | Dillon Shaffer")
        .body(html! {
            <main>
                (
                    HeaderBuilder::default()
                        .logo(header_logo())
                        .destination(Destination::new("/notes".into(), html!(<p>"Notes"</p>)))
                        .destination(Destination::new("/secrets".into(), html!(<p>"Secrets"</p>)))
                        .destination(Destination::new("/calendar".into(), html!(<p>"Calendar"</p>)))
                        .destination(Destination::new("/vulture".into(), html!(<p>"Vulture"</p>)))
                        .destination(Destination::new("/c2".into(), html!(<p>"Command & Control"</p>)))
                        .buttons(auth_buttons())
                        .build().unwrap().to_html()
                )
            </main>
        })
        .compile()
}

async fn notes_page(State(notes): State<NotesRef>) -> HtmlBundle {
    PageBuilder::default()
        .title("Notes")
        .body(html! {
            <main>
                (Header {
                    logo: Some(header_logo()),
                    destinations: vec![
                        Destination::new("/notes/new".into(), html!(<p>"New Note"</p>))
                    ],
                    buttons: auth_buttons(),
                }.to_html())
                <div>
                    for (_id, note) in notes.read().iter() {
                        let note = note.read();
                        <h2>(note.name) " | " (note.id)</h2>
                        <div>(note.description)</div>
                    }
                </div>
            </main>
        })
        .compile()
}
// <label
//   for="UserEmail"
//   class="block overflow-hidden rounded-md border border-gray-200 px-3 py-2 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600"
// >
//   <span class="text-xs font-medium text-gray-700"> Email </span>
//
//   <input
//     type="email"
//     id="UserEmail"
//     placeholder="anthony@rhcp.com"
//     class="mt-1 w-full border-none p-0 focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm"
//   />
// </label>
async fn new_note_page() -> HtmlBundle {
    PageBuilder::default()
        .title("New Note")
        .body(html! {
            <section class="bg-white">
              <div class="lg:grid lg:min-h-screen lg:grid-cols-12">
                <div class="flex items-center justify-center px-8 py-8 sm:px-12 lg:col-span-7 lg:px-16 lg:py-12 xl:col-span-6">
                  <div class="max-w-xl lg:max-w-3xl">
                    <h1 class="text-2xl">"New Note"</h1>
                    <form action="#" class="mt-8 grid grid-cols-8 gap-12">
                        <div class="col-span-12">
                            <label for="Name">"Name"</label>
                            <input type="text" id="Name" name="name"
                              class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"></input>
                        </div>
                        <div class="col-span-12">
                            <label for="Description">"Description"</label>
                            <textarea id="Description" name="description" rows="3"
                              class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"></textarea>
                        </div>
                        <div class="col-span-12">
                          <input type="submit" value="Create Note"></input>
                        </div>
                    </form>
                  </div>
                </div>
              </div>
            </section>
        })
        .compile()
}

#[derive(Deserialize)]
pub struct CreateNoteForm {
    name: String,
    description: String,
}

async fn add_note(
    State(notes): State<NotesRef>,
    Form(CreateNoteForm { name, description }): Form<CreateNoteForm>,
) -> Response {
    let mut errors = Vec::new();
    if name.trim().is_empty() {
        errors.push("Name must not be empty!");
    }

    if !errors.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            html! {
                <div id="error">
                    for error in errors {
                        <li>(error)</li>
                    }
                </div>
            }
        ).into_response();
    }

    notes.write().add_note(Note {
        id: 0,
        name,
        description,
    });

    (
        [("HX-Redirect", "/notes")],
        html!()
    ).into_response()
}

async fn note_overview_page(
    Path(id): Path<u64>,
    State(notes): State<NotesRef>,
) -> HtmlBundle {
    let Some(note) = notes.read().get_note(id) else {
        return html! {
            <div id="error">
                "no such note, id=" (id)
            </div>
        }
    };

    let note = note.read();

    PageBuilder::default()
        .title(&note.name)
        .body(html! {
            <main>
                <div class="p-6 max-w-3xl mx-auto font-sans antialiased">
                    (
                        DetailsListBuilder::default()
                            .item((html!(<p>"Name"</p>), html!(<p>(note.name)</p>)))
                            .item((html!(<p>"Description"</p>), html!(<p>(note.description)</p>)))
                            .build().unwrap().to_html()
                    )
                </div>
            </main>
        })
        .compile()
}