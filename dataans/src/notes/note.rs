use common::Config;
use common::note::{File, Id as NoteId, Note as NoteData, OwnedNote, UpdateNote};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::KeyboardEvent;
use markdown::ParseOptions;
use markdown::mdast::{Node, Text};
use time::OffsetDateTime;

use crate::common::{Attachment, Confirm, Files, TextArea};
use crate::notes::md_node::render_md_node;

#[component]
pub fn Note(
    note: NoteData<'static>,
    delete_note: SignalSetter<NoteId>,
    update_note: SignalSetter<OwnedNote>,
) -> impl IntoView {
    let config = expect_context::<RwSignal<Config>>();

    let (show_modal, set_show_modal) = signal(false);
    let (edit_mode, set_edit_mode) = signal(false);
    let (updated_note_text, set_updated_note_text) = signal(note.text.to_string());
    let (updated_files, set_updated_files) = signal(note.files.clone());

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
            let new_note = crate::backend::notes::update_note(new_note.clone())
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

    view! {
        <div
            class=move || if edit_mode.get() { "note-container note-container-edit-mode" } else { "note-container"}
            id=note.id.to_string()
            tabindex="-1"
        >
            <div class="note-meta">
                <div class="center-span">
                    {if note.created_at.as_ref() == note.updated_at.as_ref() { view! {
                        <span class="note-time">{format_date(note.created_at.as_ref())}</span>
                        <span />
                    }.into_any()} else { view! {
                        <span class="note-time" style="white-space: pre-wrap;">" UPD: "</span>
                        <span class="note-time" title=format!("Created at: {}", format_date(note.created_at.as_ref()))>
                            {format_date(note.updated_at.as_ref())}
                        </span>
                    }.into_any()}}
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
            {move || {
                let config = config.get();
                if edit_mode.get() {
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
                            {render_md_node(&md, &config.app.base_path)}
                            {move || view! { <Files files=updated_files.get() remove_file=|_| {} edit_mode=false /> }}
                        </div>
                    }.into_any()
                }}
            }
            <Show when=move || show_modal.get()>
                <Confirm
                    message="Confirm note deletion.".to_owned()
                    on_confirm=move || delete_note()
                    on_cancel=move || set_show_modal.set(false)
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
