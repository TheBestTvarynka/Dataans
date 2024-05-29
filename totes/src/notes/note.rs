use common::note::{Note as NoteData, UpdateNote};
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;
use time::OffsetDateTime;

use crate::backend::notes::{delete_note, list_notes, update_note};
use crate::common::Confirm;
use crate::notes::md_node::render_md_node;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Note(note: NoteData<'static>, set_notes: SignalSetter<Vec<NoteData<'static>>>) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (edit_mode, set_edit_mode) = create_signal(false);
    let (updated_note_text, set_updated_note_text) = create_signal(note.text.as_ref().to_owned());

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
        spawn_local(async move {
            update_note(UpdateNote {
                id: note_id,
                text: text.into(),
            })
            .await
            .expect("note updating should not fail");
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };

    let key_down = move |key| {
        if key == "Enter" {
            update_note();
        } else if key == "Escape" {
            set_edit_mode.set(false);
        }
    };

    view! {
        <div class="note-container">
            <div class="note-meta">
                <div class="horizontal note-tools">
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
                <div class="center-span">
                    <span class="note-time">{format_date(note.created_at.as_ref())}</span>
                </div>
            </div>
            {move || if edit_mode.get() {
                view! {
                    <div class="vertical">
                        <textarea
                            placeholder="New note text here..."
                            prop:value={updated_note_text}
                            on:input=move |ev| set_updated_note_text.set(event_target_value(&ev))
                            on:keydown=move |ev| key_down(ev.key())
                        />
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
            <Show when=move || show_modal.get()>
                <Confirm
                    message={"Confirm note deletion.".to_owned()}
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
