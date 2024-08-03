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

    let key_bindings = config.key_bindings.clone();

    use_hotkeys!((key_bindings.toggle_spaces_bar) => move |_| {
        set_spaces_minimized.set(!spaces_minimized.get());
    });

    use_hotkeys!((key_bindings.select_prev_space) => move |_| select_prev_space());
    use_hotkeys!((key_bindings.select_next_space) => move |_| select_next_space());

    view! {
        <div class="spaces-container">
            <Tools set_spaces spaces_minimized set_spaces_minimized config />
            <div class="spaces-scroll-area">
                {move || spaces.get().into_iter().map(|space| {
                    let selected = selected_space.get().as_ref().map(|selected| selected.id == space.id).unwrap_or_default();
                    view! { <Space space set_selected_space selected minimized={spaces_minimized} /> }
                }).collect_view()}
            </div>
        </div>
    }
}
