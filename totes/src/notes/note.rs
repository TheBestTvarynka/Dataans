use common::note::Note as NoteData;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;
use time::OffsetDateTime;

use crate::backend::notes::{delete_note, list_notes};
use crate::common::Confirm;
use crate::notes::md_node::render_md_node;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Note<'text>(note: NoteData<'text>, set_notes: SignalSetter<Vec<NoteData<'static>>>) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

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

    view! {
        <div class="note-container">
            <div class="note-meta">
                <div class="horizontal note-tools">
                    <button
                        class="tool"
                        title="Edit note"
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
            {render_md_node(&md)}
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
        "{}:{} {}/{}/{}",
        date.hour(),
        date.minute(),
        date.day(),
        date.month(),
        date.year()
    )
}
