mod space;
pub mod space_form;
pub mod tools;

use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use self::space::Space;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::{list_notes, search_notes, search_notes_in_space};
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
    let select_next_space = move || {
        if let Some(selected_space) = selected_space.get() {
            let spaces = spaces.get();
            let selected_space_index = spaces
                .iter()
                .position(|s| s.id == selected_space.id)
                .expect("selected space should present in loaded spaces");
            set_selected_space(
                spaces
                    .get(if selected_space_index + 1 == spaces.len() {
                        0
                    } else {
                        selected_space_index + 1
                    })
                    .expect("valid space index")
                    .clone(),
            );
        }
    };
    let select_prev_space = move || {
        if let Some(selected_space) = selected_space.get() {
            let spaces = spaces.get();
            let selected_space_index = spaces
                .iter()
                .position(|s| s.id == selected_space.id)
                .expect("selected space should present in loaded spaces");
            set_selected_space(
                spaces
                    .get(if selected_space_index == 0 {
                        spaces.len() - 1
                    } else {
                        selected_space_index - 1
                    })
                    .expect("valid space index")
                    .clone(),
            );
        }
    };

    let key_bindings = config.key_bindings.clone();

    use_hotkeys!((key_bindings.toggle_spaces_bar) => move |_| {
        set_spaces_minimized.set(!spaces_minimized.get());
    });

    use_hotkeys!((key_bindings.select_prev_space) => move |_| select_prev_space());
    use_hotkeys!((key_bindings.select_next_space) => move |_| select_next_space());

    let (query, set_query) = create_signal(String::new());

    let found_notes = create_resource(
        move || (query.get(), find_note_mode.get()),
        move |(query, find_note_mode)| async move {
            match find_note_mode {
                FindNoteMode::None => vec![],
                FindNoteMode::FindNote { space: Some(space) } => {
                    if query.is_empty() {
                        vec![]
                    } else {
                        search_notes_in_space(space.id, &query)
                            .await
                            .expect("Notes searching should not fail")
                    }
                }
                FindNoteMode::FindNote { space: None } => {
                    if query.is_empty() {
                        vec![]
                    } else {
                        search_notes(&query).await.expect("Notes searching should not fail")
                    }
                }
            }
        },
    );

    view! {
        <div class="spaces-container">
            <Tools set_spaces spaces_minimized set_spaces_minimized set_find_node_mode set_query=set_query.into() config />
            <div class="spaces-scroll-area">
                {move || match find_note_mode.get() {
                    FindNoteMode::None => spaces.get().into_iter().map(|space| {
                        let selected = selected_space.get().as_ref().map(|selected| selected.id == space.id).unwrap_or_default();
                        view! { <Space space set_selected_space selected minimized={spaces_minimized} /> }
                    }).collect_view(),
                    FindNoteMode::FindNote { space } => {
                        use_hotkeys!(("Escape") => move |_| set_find_node_mode.set(FindNoteMode::None));
                        let mut elems = Vec::new();
                        elems.push(if let Some(space) = space {
                            view! {
                                <div class="note-search-options">
                                    <span class="note-search-label">"Search notes in:"</span>
                                    <Space space set_selected_space selected=true minimized={spaces_minimized} />
                                </div>
                            }.into_any()
                        } else {
                            view! { <div /> }.into_any()
                        });
                        elems.into_iter().collect_view()
                    },
                }}
                {move || if find_note_mode.get().is_find_mode() { view! {
                    <Suspense
                        fallback=move || view! { <span>"Loading notes..."</span> }
                    >
                        {move || found_notes.get()
                            .map(|notes| notes.into_iter().map(|note| view! {
                                <span>{note.text.to_string()}</span>
                            }).collect_view())}
                    </Suspense>
                }} else {
                    view! {}.into_view()
                }}
            </div>
        </div>
    }
}
