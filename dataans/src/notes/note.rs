use common::note::{File, Id as NoteId, Note as NoteData, UpdateNote};
use leptos::web_sys::KeyboardEvent;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;
use time::OffsetDateTime;

use crate::backend::file::remove_file;
use crate::common::{Attachment, Confirm, Files, TextArea};
use crate::notes::md_node::render_md_node;

#[component]
pub fn Note(
    note: NoteData<'static>,
    delete_note: SignalSetter<NoteId>,
    update_note: SignalSetter<UpdateNote<'static>>,
) -> impl IntoView {
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
    let delete_note = move || {
        spawn_local(async move {
            crate::backend::notes::delete_note(note_id)
                .await
                .expect("note deletion should not fail");
            delete_note.set(note_id);
        });
    };

    let update_note_fn = move || {
        let text = updated_note_text.get();
        let files = updated_files.get();

        spawn_local(async move {
            let new_note = UpdateNote {
                id: note_id,
                text: text.into(),
                files,
            };
            crate::backend::notes::update_note(new_note.clone())
                .await
                .expect("note updating should not fail");
            update_note.set(new_note);
        });
    };

    let remove_file_locally = move |file: File| {
        let id = file.id;
        let mut files = updated_files.get();
        files.retain(|file| file.id != id);
        set_updated_files.set(files.clone());
    };

    let remove_file = move |file: File| {
        let text = updated_note_text.get();
        let mut files = updated_files.get();
        let id = file.id;

        spawn_local(async move {
            remove_file(&file.path).await;

            files.retain(|file| file.id != id);
            set_updated_files.set(files.clone());

            let new_note = UpdateNote {
                id: note_id,
                text: text.into(),
                files,
            };
            crate::backend::notes::update_note(new_note.clone())
                .await
                .expect("note updating should not fail");
            update_note.set(new_note);
        });
    };

    view! {
        <div
            class=move || if edit_mode.get() { "note-container note-container-edit-mode" } else { "note-container"}
            id=note.id.to_string()
            tabindex="-1"
        >
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
                        <img alt="edit note" src="/public/icons/edit-space.svg" />
                    </button>
                    <button
                        class="tool"
                        title="Delete note"
                        on:click=move |_| set_show_modal.set(true)
                    >
                        <img alt="delete note" src="/public/icons/delete-space.png" />
                    </button>
                </div>
            </div>
            {move || if edit_mode.get() {
                let note_files = note.files.clone();
                let key_down = move |key: KeyboardEvent| {
                    if key.key() == "Enter" && !key.shift_key() {
                        update_note_fn();
                    } else if key.key() == "Escape" {
                        set_updated_files.set(note_files.clone());
                        set_edit_mode.set(false);
                    }
                };

                let note_files = note.files.clone();
                let cancel = move |_| {
                    set_updated_files.set(note_files.clone());
                    set_edit_mode.set(false);
                };

                view! {
                    <div class="vertical">
                        <TextArea
                            id=format!("edit_input_{}", note.id)
                            text=updated_note_text.into()
                            set_text=move |t| set_updated_note_text.set(t)
                            key_down
                        />
                        <div class="horizontal">
                            <button
                                class="tool"
                                title="Discard changed"
                                on:click=cancel
                            >
                                <img alt="discard" src="/public/icons/cancel.png" />
                            </button>
                            <button
                                class="tool"
                                title="Save changes"
                                on:click=move |_| update_note_fn()
                            >
                                <img alt="save" src="/public/icons/accept.png" />
                            </button>
                            <Attachment id=note_id.to_string() files=updated_files.into() set_files=move |files| set_updated_files.set(files) />
                        </div>
                        {move || view! { <Files files=updated_files.get() remove_file=remove_file_locally edit_mode=true /> }}
                    </div>
                }.into_any()
            } else {
                view !{
                    <div class="vertical">
                        {render_md_node(&md)}
                        {move || view! { <Files files=updated_files.get() remove_file edit_mode=false /> }}
                    </div>
                }.into_any()
            }}
            <Show when=move || show_modal.get()>
                <Confirm
                    message="Confirm note deletion.".to_owned()
                    on_confirm=move |_| delete_note()
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
