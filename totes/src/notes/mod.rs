pub mod editor;
mod info;
mod md_node;
mod note;

use common::note::UpdateNote;
use common::space::Space as SpaceData;
use leptos::*;

use self::editor::Editor;
use self::info::Info;
use self::note::Note;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;

#[component]
pub fn Notes() -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();

    let (current_space, _) = create_slice(
        global_state,
        |state| state.selected_space.clone(),
        |state, space| state.selected_space = Some(space),
    );

    let (_, set_spaces) = create_slice(
        global_state,
        |_state| (),
        |state, spaces: Vec<SpaceData>| {
            if let Some(selected_space) = state.selected_space.as_mut() {
                let selected_space_id = selected_space.id;
                if let Some(updated_space) = spaces.iter().find(|s| s.id == selected_space_id) {
                    *selected_space = updated_space.clone();
                } else {
                    state.selected_space = None;
                }
            }
            state.spaces = spaces;
        },
    );

    let (notes, set_notes) = create_slice(
        global_state,
        |state| state.notes.clone(),
        |state, notes| state.notes = notes,
    );

    let (_, delete_note) = create_slice(
        global_state,
        |_state| (),
        |state, note_id| state.notes.retain(|note| note.id != note_id),
    );

    let (_, update_note) = create_slice(
        global_state,
        |_state| (),
        |state, new_note: UpdateNote<'static>| {
            if let Some(note) = state.notes.iter_mut().find(|note| note.id == new_note.id) {
                note.text = new_note.text;
                note.files = new_note.files;
            }
        },
    );

    let _ = move || {
        if let Some(space) = current_space.get() {
            spawn_local(async move {
                set_notes.set(list_notes(space.id).await.expect("Notes listing should not fail"));
            });
        }
    };

    view! {
        <div class="notes-container">
            <Show
                when=move || current_space.get().is_some()
                fallback=|| view! { <div /> }
            >
                <Info current_space=current_space.get().unwrap() set_spaces />
            </Show>
            <div class="notes-inner">
                <div class="notes">
                    {move || notes
                        .get()
                        .iter()
                        .rev()
                        .cloned()
                        .map(|note| view! { <Note note set_notes delete_note update_note /> })
                        .collect::<Vec<_>>()
                    }
                </div>
                <Show when=move || current_space.get().is_some()>
                    <Editor space_id=current_space.get().as_ref().unwrap().id set_notes />
                </Show>
            </div>
        </div>
    }
}
