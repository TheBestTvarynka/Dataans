use common::note::Note;
use common::space::Id as SpaceId;
use js_sys::{ArrayBuffer, Uint8Array};
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlInputElement, KeyboardEvent};

use crate::backend::notes::{create_note, list_notes};
use crate::common::{Files, TextArea};

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

    let handle_files_upload = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            let files = (0..files.length())
                .map(|index| {
                    let file = files.get(index).unwrap();
                    let blob = file.slice().expect("File reading should not fail");
                    let name = file.name();

                    async { (name.clone(), upload_file(blob, name).await) }
                })
                .collect::<Vec<_>>();

            let files = futures::future::join_all(files);
            spawn_local(async move {
                let files = files
                    .await
                    .into_iter()
                    .map(|(name, path)| common::note::File {
                        name,
                        path: path.into(),
                    })
                    .collect();
                set_files.set(files);
            });
        };
    };

    view! {
        <div class="editor-container">
            <TextArea id="create_note".to_owned() text=note set_text=move |t| set_note.set(t) key_down />
            <div class="editor-meta">
                {move || view!{ <Files files=files.get() /> }}
                <button class="tool">
                    <label for="note-files">
                        <img alt="attach file" src="/public/icons/attachment.png" />
                    </label>
                </button>
                <input id="note-files" type="file" multiple=true style="display: none" on:input=move |ev| handle_files_upload(ev) />
                <button on:click=move |_| create_note() title="Create note" class="tool">
                    <img alt="create note" src="/public/icons/create-note.png" />
                </button>
            </div>
        </div>
    }
}

// Returns path to the uploaded file.
async fn upload_file(blob: Blob, name: String) -> String {
    let file_raw_data = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
        .await
        .expect("File reading should not fail");

    let file_raw_data = file_raw_data
        .dyn_into::<ArrayBuffer>()
        .expect("Expected an ArrayBuffer");
    let file_raw_data = Uint8Array::new(&file_raw_data);

    let mut file_bytes = vec![0; file_raw_data.length() as usize];
    file_raw_data.copy_to(file_bytes.as_mut_slice());

    crate::backend::file::upload_file(&name, &file_bytes).await
}
