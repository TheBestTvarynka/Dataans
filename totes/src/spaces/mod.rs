mod space;
pub mod space_form;
mod tools;

use common::space::OwnedSpace;
use leptos::*;
use leptos_hotkeys::use_hotkeys;

use self::space::Space;
use self::tools::Tools;
use crate::app::GlobalState;
use crate::backend::notes::list_notes;
use crate::backend::spaces::list_spaces;

#[component]
pub fn Spaces() -> impl IntoView {
    let global_state = expect_context::<RwSignal<GlobalState>>();

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

    use_hotkeys!(("ControlLeft+keyS") => move |_| {
        set_spaces_minimized.set(!spaces_minimized.get());
    });

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
    });

    view! {
        <div class="spaces-container">
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
