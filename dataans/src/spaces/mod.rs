mod found_notes_list;
mod space;
pub mod space_form;
mod spaces_list;
pub mod tools;

use common::note::Id as NoteId;
use common::profile::{Sync, SyncMode, UserContext};
use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::use_hotkeys;

use self::found_notes_list::FoundNotesList;
use self::space::Space;
use self::spaces_list::SpacesList;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::backend::spaces::list_spaces;
use crate::backend::sync::trigger_full_sync;
use crate::utils::focus_element;
use crate::FindNoteMode;

#[component]
pub fn Spaces(spaces: Signal<Vec<OwnedSpace>>, set_spaces: SignalSetter<Vec<OwnedSpace>>) -> impl IntoView {
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

    let (query, set_query) = create_signal(String::new());

    let toaster = leptoaster::expect_toaster();

    let app_info_window_toaster = toaster.clone();
    let show_app_info_window = move |_| {
        let t = app_info_window_toaster.clone();
        spawn_local(async move {
            try_exec!(
                crate::backend::window::show_app_info_window().await,
                "Failed to create auth window",
                t
            );
        });
    };

    let user_context = expect_context::<RwSignal<Option<UserContext>>>();

    let global_config = expect_context::<RwSignal<Config>>();

    view! {
        <div class="spaces-container">
            {move || view! { <Tools set_spaces spaces_minimized set_spaces_minimized set_find_node_mode set_query=set_query.into() set_selected_space config=global_config.get() /> }}
            {move || {
                let config = global_config.get();
                match find_note_mode.get() {
                    FindNoteMode::None => view!{
                        <SpacesList config selected_space spaces spaces_minimized set_selected_space />
                    },
                    FindNoteMode::FindNote { space } => {
                        use_hotkeys!(("Escape") => move |_| set_find_node_mode.set(FindNoteMode::None));
                        view! {
                            <FoundNotesList config query search_in_space=space spaces_minimized focus_note />
                        }
                    },
                }
            }}
            <div style="flex-grow: 1; align-content: end; display: flex; flex-direction: column; align-items: center; justify-content: flex-end;">
                {move || if let Some(UserContext { sync_config: Sync::Enabled { mode: SyncMode::Manual, .. }, .. }) = user_context.get() {
                    let sync_toaster = toaster.clone();
                    let start_full_sync = move |_| {
                        let t = sync_toaster.clone();
                        spawn_local(async move {
                            try_exec!(
                                trigger_full_sync().await,
                                "Failed to start syncing...",
                                t
                            );
                        });
                    };

                    view!{
                        <button title="Sync data" class="tool">
                            <img alt="sync-icon" src="/public/icons/synchronize-light.png" on:click=start_full_sync />
                        </button>
                    }.into_any()
                } else {
                    view! { <span /> }.into()
                }}
                <div style="display: inline-flex; width: 100%; justify-content: center; margin-bottom: 0.2em;">
                    <button class="button_cancel" on:click=show_app_info_window>
                        {format!("{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"))}
                    </button>
                </div>
            </div>
        </div>
    }
}
