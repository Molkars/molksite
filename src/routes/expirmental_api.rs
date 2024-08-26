use crate::html::HtmlBundle;

struct UiState {
    view: View,

}

enum View {
    Notes(notes::NotesView)
}


impl Into<HtmlBundle> for UiState {
    fn into(self) -> HtmlBundle {
        todo!()
    }
}

mod notes {
    pub struct NotesView {

    }

    pub enum Drawer {
        NewNote(NewNoteState),
        NoteDetail(NoteDetailState),
    }

    struct NewNoteState {

    }

    struct NoteDetailState {

    }
}