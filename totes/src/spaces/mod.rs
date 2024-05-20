mod space;
mod tools;

use leptos::*;

use self::space::Space;
use self::tools::Tools;
use crate::app::GlobalState;
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

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
    });

    view! {
        <div class="spaces-container">
            <Tools set_spaces />
            <div class="spaces">
                {move || spaces.get().iter().cloned().map(|space| {
                    let selected = selected_space.get().as_ref().map(|selected| selected.id == space.id).unwrap_or_default();
                    view! { <Space space set_selected_space selected /> }
                }).collect_view()}
            </div>
        </div>
    }
}
