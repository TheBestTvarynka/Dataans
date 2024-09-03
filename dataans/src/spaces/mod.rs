mod app_info;
mod found_notes_list;
mod space;
pub mod space_form;
mod spaces_list;
pub mod tools;

use common::note::Id as NoteId;
use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use self::app_info::AppInfo;
use self::found_notes_list::FoundNotesList;
use self::space::Space;
use self::spaces_list::SpacesList;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::backend::spaces::list_spaces;
use crate::utils::focus_element;
use crate::FindNoteMode;

#[component]
pub fn Spaces(
    config: Config,
    spaces: Signal<Vec<OwnedSpace>>,
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
) -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("loaded spaces"));
    });

    let (selected_space, set_selected_space_s) = create_slice(
        global_state,
        |state| state.selected_space.clone(),
        |state, space| state.selected_space = Some(space),
    );
    let (_, set_notes) = create_slice(global_state, |_state| (), |state, notes| state.notes = notes);
    let (find_note_mode, set_find_node_mode) = create_slice(
        global_state,
        |state| state.find_note_mode.clone(),
        |state, find_note_mode| state.find_note_mode = find_note_mode,
    );
    let set_selected_space = move |space: OwnedSpace| {
        let space_id = space.id;
        set_selected_space_s.set(space);
        spawn_local(async move {
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
        });
    };
    let focus_note = move |(note_id, space): (NoteId, OwnedSpace)| {
        let space_id = space.id;
        set_selected_space_s.set(space);
        spawn_local(async move {
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
            focus_element(note_id.to_string());
        });
    };
    let (spaces_minimized, set_spaces_minimized) = create_slice(
        global_state,
        |state| state.minimize_spaces,
        |state, minimized| state.minimize_spaces = minimized,
    );

    let toggle_spaces_bar = config.key_bindings.toggle_spaces_bar.clone();
    use_hotkeys!((toggle_spaces_bar) => move |_| {
        set_spaces_minimized.set(!spaces_minimized.get());
    });

    let (query, set_query) = create_signal(String::new());

    view! {
        <div class="spaces-container">
            <Tools set_spaces spaces_minimized set_spaces_minimized set_find_node_mode set_query=set_query.into() config=config.clone() />
            {move || match find_note_mode.get() {
                FindNoteMode::None => view!{
                    <SpacesList config=config.clone() selected_space spaces spaces_minimized set_selected_space />
                },
                FindNoteMode::FindNote { space } => {
                    use_hotkeys!(("Escape") => move |_| set_find_node_mode.set(FindNoteMode::None));
                    view! {
                        <FoundNotesList config=config.clone() query search_in_space=space spaces_minimized focus_note />
                    }
                },
            }}
            <div style="flex-grow: 1; align-content: end;">
                {move || if spaces_minimized.get() {
                    view! {
                        <a class="icons-by-icons8" href="https://icons8.com" target="_blank">
                            <svg width="18" height="18" viewBox="0 0 18 18">
                                <path d="M9 0H0V18H9V0Z" fill="#1FB141"></path>
                                <path d="M13.5 9C15.9853 9 18 6.98528 18 4.5C18 2.01472 15.9853 0 13.5 0C11.0147 0 9 2.01472 9 4.5C9 6.98528 11.0147 9 13.5 9Z" fill="#1FB141"></path>
                                <path d="M13.5 18C15.9853 18 18 15.9853 18 13.5C18 11.0147 15.9853 9 13.5 9C11.0147 9 9 11.0147 9 13.5C9 15.9853 11.0147 18 13.5 18Z" fill="#1FB141"></path>
                            </svg>
                        </a>
                    }.into_any()
                } else {
                    view! {
                        <span class="icons-by-icons8">"Icons by: "<a href="https://icons8.com" target="_blank">"icons8.com"</a></span>
                    }.into_any()
                }}
                <AppInfo />
            </div>
        </div>
    }
}
