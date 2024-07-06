use common::note::{Note as NoteData, UpdateNote, File};
use leptos::web_sys::KeyboardEvent;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;
use time::OffsetDateTime;

use crate::backend::file::remove_file;
use crate::backend::notes::{delete_note, list_notes, update_note};
use crate::common::{Confirm, Files, TextArea};
use crate::notes::md_node::render_md_node;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Note(note: NoteData<'static>, set_notes: SignalSetter<Vec<NoteData<'static>>>) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (edit_mode, set_edit_mode) = create_signal(false);
    let (updated_note_text, set_updated_note_text) = create_signal(note.text.to_string());
    let (updated_files, set_updated_files) = create_signal(note.files.clone());

    let md = markdown::to_mdast(note.text.as_ref(), &ParseOptions::gfm()).unwrap_or_else(|_| {
        Node::Text(Text {
            value: "Can not parse MD message".into(),
            position: None,
        })
    });

    let note_id = note.id;
    let space_id = note.space_id;
    let delete_note = move || {
        spawn_local(async move {
            delete_note(note_id).await.expect("note deletion should not fail");
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };

    let update_note = move || {
        let text = updated_note_text.get();
        let files = updated_files.get();

        spawn_local(async move {
            update_note(UpdateNote {
                id: note_id,
                text: text.into(),
                files,
            })
            .await
            .expect("note updating should not fail");
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };

    let remove_file = move |file: File| {
        let text = updated_note_text.get();
        let mut files = updated_files.get();
        let id = file.id;

        spawn_local(async move {
            remove_file(&file.path).await;

            files.retain(|file| file.id != id);
            set_updated_files.set(files.clone());

            crate::backend::notes::update_note(UpdateNote {
                id: note_id,
                text: text.into(),
                files,
            })
            .await
            .expect("note updating should not fail");
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };

    let key_down = move |key: KeyboardEvent| {
        if key.key() == "Enter" && !key.shift_key() {
            update_note();
        } else if key.key() == "Escape" {
            set_edit_mode.set(false);
        }
    };

    view! {
        <div class="note-container">
            <div class="note-meta">
                <div class="center-span">
                    <span class="note-time">{format_date(note.created_at.as_ref())}</span>
                </div>
                <div class="note-tools">
                    <button
                        class="tool"
                        title="Edit note"
                        on:click=move |_| set_edit_mode.set(true)
                    >
                        <img alt="change space name" src="/public/icons/edit-space.svg" />
                    </button>
                    <button
                        class="tool"
                        title="Delete note"
                        on:click=move |_| set_show_modal.set(true)
                    >
                        <img alt="delete space" src="/public/icons/delete-space.png" />
                    </button>
                </div>
            </div>
            {move || if edit_mode.get() {
                view! {
                    <div class="vertical">
                        <TextArea id=note.id.to_string() text=updated_note_text set_text=move |t| set_updated_note_text.set(t) key_down />
                        <div class="horizontal">
                            <button
                                class="tool"
                                title="Discard changed"
                                on:click=move |_| set_edit_mode.set(false)
                            >
                                <img alt="discard" src="/public/icons/cancel.png" />
                            </button>
                            <button
                                class="tool"
                                title="Save changes"
                                on:click=move |_| update_note()
                            >
                                <img alt="save" src="/public/icons/accept.png" />
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                render_md_node(&md)
            }}
            <Files files={note.files} remove_file />
            <Show when=move || show_modal.get()>
                <Confirm
                    message="Confirm note deletion.".to_owned()
                    on_confirm=move || delete_note()
                    on_cancel=move |_| set_show_modal.set(false)
                />
            </Show>
        </div>
    }
}

fn format_date(date: &OffsetDateTime) -> String {
    format!(
        "{:02}:{:02}:{:02} {:02}/{}/{:04}",
        date.hour(),
        date.minute(),
        date.second(),
        date.day(),
        date.month(),
        date.year()
    )
}
