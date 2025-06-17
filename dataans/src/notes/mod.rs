pub mod editor;
mod info;
pub mod md_node;
mod note;
pub mod note_preview;

use common::note::OwnedNote;
use common::space::{OwnedSpace, Space as SpaceData};
use common::Config;
use leptos::*;
use wasm_bindgen::JsCast;

use self::editor::Editor;
use self::info::Info;
use self::note::Note;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::spaces::tools::SEARCH_NOTE_INPUT_ID;
use crate::utils::focus_element;
use crate::FindNoteMode;

#[component]
pub fn Notes() -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();
    let config = expect_context::<RwSignal<Config>>();

    let (current_space, set_selected_space_s) = create_slice(
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

    let (_, set_find_node_mode) = create_slice(
        global_state,
        |_state| (),
        |state, find_note_mode| state.find_note_mode = find_note_mode,
    );

    let (_, set_spaces_minimized) = create_slice(
        global_state,
        |state| state.minimize_spaces,
        |state, minimized| state.minimize_spaces = minimized,
    );

    let (notes, set_notes) = create_slice(
        global_state,
        |state| state.notes.clone(),
        |state, notes| state.notes = notes,
    );

    let set_selected_space = move |space: OwnedSpace| {
        let space_id = space.id;
        set_selected_space_s.set(space);
        spawn_local(async move {
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };

    let (_, delete_note) = create_slice(
        global_state,
        |_state| (),
        |state, note_id| state.notes.retain(|note| note.id != note_id),
    );
    let (_, delete_state_space) = create_slice(
        global_state,
        |_state| (),
        |state, space_id| {
            state.notes.clear();
            state.selected_space = None;
            state.spaces.retain(|space| space.id != space_id);
        },
    );

    let (_, create_note) = create_slice(global_state, |_state| (), |state, new_note| state.notes.push(new_note));

    let create_note = move |note| {
        create_note.set(note);
        if let Some(notes_div) = document().get_element_by_id("notes") {
            let notes = notes_div
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Expected HtmlElement");
            notes.set_scroll_top(notes.scroll_height());
        } else {
            warn!("notes component is not present.");
        }
    };

    let (_, update_note) = create_slice(
        global_state,
        |_state| (),
        |state, new_note: OwnedNote| {
            if let Some(note) = state.notes.iter_mut().find(|note| note.id == new_note.id) {
                *note = new_note
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
                {move || view! {
                    <Info
                        current_space=current_space.get().unwrap()
                        set_spaces
                        delete_state_space
                        toggle_note_search=move |_| {
                            set_spaces_minimized.set(false);
                            set_find_node_mode.set(FindNoteMode::FindNote {
                                space: Some(current_space.get().unwrap()),
                            });
                            focus_element(SEARCH_NOTE_INPUT_ID);
                        }
                        set_selected_space
                        config=config.get()
                    />
                }}
            </Show>
            <div class="notes-inner">
                <div class="notes" id="notes">
                    {move || notes
                        .get()
                        .iter()
                        .rev()
                        .cloned()
                        .map(|note| view! { <Note note delete_note update_note /> })
                        .collect::<Vec<_>>()
                    }
                </div>
                <Show when=move || current_space.get().is_some()>
                    <Editor space_id=current_space.get().as_ref().unwrap().id create_note />
                </Show>
            </div>
        </div>
    }
}
