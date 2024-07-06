use common::note::{File, Note};
use common::space::Id as SpaceId;
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::backend::file::remove_file;
use crate::backend::notes::{create_note, list_notes};
use crate::common::{Attachment, Files, TextArea};

#[component]
pub fn Editor(space_id: SpaceId, set_notes: SignalSetter<Vec<Note<'static>>>) -> impl IntoView {
    let (note, set_note) = create_signal(String::new());
    let (files, set_files) = create_signal(Vec::new());

    let create_note = move || {
        let note_text = note.get();
        if note_text.trim().is_empty() {
            return;
        }

        set_note.set(String::new());

        let files = files.get();
        set_files.set(Vec::new());

        spawn_local(async move {
            create_note(Note {
                id: Uuid::new_v4().into(),
                text: note_text.trim().into(),
                created_at: OffsetDateTime::now_utc().into(),
                space_id,
                files,
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

    let remove_file = move |File { id, name: _, path }| {
        let mut files = files.get();
        spawn_local(async move {
            remove_file(&path).await;

            files.retain(|file| file.id != id);
            set_files.set(files);
        });
    };

    let handle_files = move |files| {
        set_files.set(files);
    };

    view! {
        <div class="editor-container">
            <TextArea id="create_note".to_owned() text=note set_text=move |t| set_note.set(t) key_down />
            <div class="editor-meta">
                {move || view!{ <Files files=files.get() remove_file /> }}
                <Attachment id="new-note-files".to_string() files set_files=handle_files />
                <button on:click=move |_| create_note() title="Create note" class="tool">
                    <img alt="create note" src="/public/icons/create-note.png" />
                </button>
            </div>
        </div>
    }
}
