mod found_notes_list;
mod space;
pub mod space_form;
mod spaces_list;
pub mod tools;

use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use self::found_notes_list::FoundNotesList;
use self::space::Space;
use self::spaces_list::SpacesList;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::backend::spaces::list_spaces;
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

    let (selected_space, set_selected_space) = create_slice(
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
        set_selected_space.set(space);
        spawn_local(async move {
            set_notes.set(list_notes(space_id).await.expect("Notes listing should not fail"));
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
            <Tools set_spaces spaces_minimized set_spaces_minimized set_find_node_mode set_query=set_query.into() config={config.clone()} />
            {move || match find_note_mode.get() {
                FindNoteMode::None => view!{
                    <SpacesList config={config.clone()} selected_space spaces spaces_minimized set_selected_space />
                },
                FindNoteMode::FindNote { space } => {
                    use_hotkeys!(("Escape") => move |_| set_find_node_mode.set(FindNoteMode::None));
                    view! {
                        <FoundNotesList config={config.clone()} query search_in_space={space} spaces_minimized />
                    }
                },
            }}
        </div>
    }
}
