use common::note::Note;
use common::space::Id as SpaceId;
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::backend::notes::{create_note, list_notes};

#[component]
pub fn Editor(space_id: SpaceId, set_notes: SignalSetter<Vec<Note<'static>>>) -> impl IntoView {
    let (note, set_note) = create_signal(String::new());

    let create_note = move || {
        let note_text = note.get();
        if note_text.trim().is_empty() {
            return;
        }
        set_note.set(String::new());
        spawn_local(async move {
            create_note(Note {
                id: Uuid::new_v4().into(),
                text: note_text.trim().into(),
                created_at: OffsetDateTime::now_utc().into(),
                space_id,
            })
            .await
            .expect("Note creating should not fail.");
            set_notes.set(list_notes(space_id).await.expect("Note listing should not fail"));
        });
    };

    let key_down = move |key| {
        if key == "Enter" {
            create_note();
        }
    };

    view! {
        <div class="editor">
            <input
                type="text"
                placeholder="Type a note..."
                class="input"
                on:input=move |ev| set_note.set(event_target_value(&ev))
                on:keydown=move |ev| key_down(ev.key())
                prop.value=move || note.get()
                value=move || note.get()
            />
            <button on:click=move |_| create_note()>
                "Send"
            </button>
        </div>
    }
}
