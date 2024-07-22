mod space;
pub mod space_form;
mod tools;

use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use self::space::Space;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::backend::spaces::list_spaces;

#[component]
pub fn Spaces() -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();

    let config = expect_context::<RwSignal<Config>>();
    let (key_bindings, _) = create_slice(config, |config| config.key_bindings.clone(), |_config, _: ()| {});

    let (spaces, set_spaces) = create_slice(
        global_state,
        |state| state.spaces.clone(),
        |state, spaces| state.spaces = spaces,
    );
    let (selected_space, set_selected_space) = create_slice(
        global_state,
        |state| state.selected_space.clone(),
        |state, space| state.selected_space = Some(space),
    );
    let (_, set_notes) = create_slice(
        global_state,
        |state| state.notes.clone(),
        |state, notes| state.notes = notes,
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

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
    });

    view! {
        <div class="spaces-container">
            {move || {
                let key_bindings = key_bindings.get();

                use_hotkeys!((key_bindings.toggle_spaces_bar) => move |_| {
                    set_spaces_minimized.set(!spaces_minimized.get());
                });

                use_hotkeys!(("AltLeft+Digit1") => move |_| select_prev_space());
                use_hotkeys!(("AltLeft+Digit2") => move |_| select_next_space());

                view! {}
            }}
            <Tools set_spaces spaces_minimized set_spaces_minimized />
            <div class="spaces">
                {move || spaces.get().iter().cloned().map(|space| {
                    let selected = selected_space.get().as_ref().map(|selected| selected.id == space.id).unwrap_or_default();
                    view! { <Space space set_selected_space selected minimized={spaces_minimized} /> }
                }).collect_view()}
            </div>
        </div>
    }
}
