use common::note::{DraftNote, File, Note};
use common::space::Id as SpaceId;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::backend::file::remove_file;
use crate::common::{Attachment, Files, TextArea};

#[component]
pub fn Editor(space_id: SpaceId, #[prop(into)] create_note: Callback<Note<'static>, ()>) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (draft_note, set_draft_note) =
        if let Ok(draft_note) = LocalStorage::get::<DraftNote>(space_id.inner().to_string()) {
            create_signal(draft_note)
        } else {
            create_signal(DraftNote::default())
        };

    let set_draft_note = move |draft_note| {
        if let Err(err) = LocalStorage::set(space_id.inner().to_string(), &draft_note) {
            error!(
                "Cannot save note in local storage. err={:?}, draft_note={:?}",
                err, draft_note
            );
        }
        set_draft_note.set(draft_note);
    };

    let create_note = move || {
        let DraftNote { text: note_text, files } = draft_note.get();

        if note_text.as_ref().trim().is_empty() {
            return;
        }

        set_draft_note(DraftNote::default());

        spawn_local(async move {
            let new_note = Note {
                id: Uuid::new_v4().into(),
                text: note_text.as_ref().trim().to_string().into(),
                created_at: OffsetDateTime::now_utc().into(),
                space_id,
                files,
            };
            crate::backend::notes::create_note(new_note.clone())
                .await
                .expect("Note creating should not fail.");
            create_note.call(new_note);
        });
    };

    let key_down = move |key: KeyboardEvent| {
        if key.key() == "Enter" && !key.shift_key() {
            key.prevent_default();
            create_note();
        }
    };

    let toaster = toaster.clone();
    let remove_file = Callback::new(move |File { id, name: _, path: _ }| {
        let toaster = toaster.clone();

        let DraftNote { text, mut files } = draft_note.get();

        spawn_local(async move {
            try_exec!(remove_file(id).await, "File removing failed", toaster);

            files.retain(|file| file.id != id);
            set_draft_note(DraftNote { text, files });
        });
    });

    let handle_files = move |files| {
        if let Some(DraftNote { text, files: _ }) = draft_note.try_get_untracked() {
            set_draft_note(DraftNote { text, files });
        }
    };

    let set_text = move |text: String| {
        if let Some(DraftNote { text: _, files }) = draft_note.try_get_untracked() {
            set_draft_note(DraftNote {
                text: text.into(),
                files,
            });
        }
    };

    view! {
        <div class="editor-container">
            <div class="horizontal">
                <TextArea
                    id="create_note".to_owned()
                    text=Signal::derive(move || draft_note.get().text.to_string())
                    set_text
                    key_down
                />
                <div style="display: inline-flex; align-items: center; padding: 0.3em; align-self: flex-end;">
                    <Attachment
                        id="new-note-files".to_string()
                        files=Signal::derive(move || draft_note.get().files)
                        set_files=handle_files
                    />
                    <button on:click=move |_| create_note() title="Create note" class="tool">
                        <img alt="create note" src="/public/icons/create-note.png" />
                    </button>
                </div>
            </div>
            <div class="editor-meta">
                {move || view!{ <Files files=draft_note.get().files.clone() remove_file edit_mode=true /> }}
            </div>
        </div>
    }
}
