use common::note::{Id as NoteId, NoteFullOwned};
use leptos::*;

#[component]
pub fn NotePreview(
    note: NoteFullOwned,
    minimized: Signal<bool>,
    selected: bool,
    #[prop(into)] set_selected_note: Callback<NoteId, ()>,
) -> impl IntoView {
    let class = if selected {
        "note-preview note-preview-selected"
    } else {
        "note-preview"
    };

    let note_id = note.id;

    view! {
        <div class=class>
            <img class="note-preview-image" alt="space avatar image" src=note.space.avatar.to_string() />
            <Show when=move || !minimized.get()>
                <div class="vertical" on:click=move |_| set_selected_note.call(note_id)>
                    <span class="note-preview-space-name">{note.space.name.to_string()}</span>
                    <span class="note-preview-note-text">{note_preview_text(note.text.as_ref())}</span>
                </div>
            </Show>
        </div>
    }
}

fn note_preview_text(text: &str) -> String {
    text.chars().map(|c| if c == '\n' { ' ' } else { c }).take(30).collect()
}
