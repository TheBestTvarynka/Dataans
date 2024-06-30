use common::note::Note;
use common::space::Id as SpaceId;
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::backend::notes::{create_note, list_notes};
use crate::common::TextArea;

#[component]
pub fn Editor(space_id: SpaceId, set_notes: SignalSetter<Vec<Note<'static>>>) -> impl IntoView {
    let (note, set_note) = create_signal(String::new());

    let create_note = move || {
        let note_text = note.get();
        if note_text.trim().is_empty() {
            return;
        }
        set_note.set(String::new());
        spawn_local(async move {
            create_note(Note {
                id: Uuid::new_v4().into(),
                text: note_text.trim().into(),
                created_at: OffsetDateTime::now_utc().into(),
                space_id,
            })
            .await
            .expect("Note creating should not fail.");
            set_notes.set(list_notes(space_id).await.expect("Note listing should not fail"));
        });
    };

    let key_down = move |key: KeyboardEvent| {
        if key.key() == "Enter" && !key.shift_key() {
            create_note();
        }
    };

    view! {
        <div class="editor-container">
            <TextArea id="create_note".to_owned() text={note} set_text=move |t| set_note.set(t) key_down />
            <button on:click=move |_| create_note() title="Create note" class="create-note-button tool">
                <img alt="create note" src="/public/icons/create-note.png" />
            </button>
        </div>
    }
}
