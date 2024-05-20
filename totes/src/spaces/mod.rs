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

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
    });

    view! {
        <div class="spaces-container">
            <Tools set_spaces />
            <div class="spaces">
                {move || spaces.get().iter().cloned().map(|space| view! {
                    <Space space={space} />
                }).collect_view()}
            </div>
        </div>
    }
}
