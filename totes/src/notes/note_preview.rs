use common::note::NoteFullOwned;
use leptos::*;

#[component]
pub fn NotePreview(note: NoteFullOwned, minimized: Signal<bool>) -> impl IntoView {
    view! {
        <div class="note-preview">
            <img class="note-preview-image" alt="space avatar image" src=note.space.avatar.to_string() />
            <Show when=move || !minimized.get()>
                <div class="vertical">
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
