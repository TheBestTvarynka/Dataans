use common::space::OwnedSpace;
use common::Config;
use leptos::callback::Callback;
use leptos::prelude::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use crate::spaces::Space;

#[component]
pub fn SpacesList(
    config: Config,
    selected_space: Signal<Option<OwnedSpace>>,
    spaces: Signal<Vec<OwnedSpace>>,
    spaces_minimized: Signal<bool>,
    #[prop(into)] set_selected_space: Callback<OwnedSpace, ()>,
) -> impl IntoView {
    let select_next_space = move || {
        if let Some(selected_space) = selected_space.get() {
            let spaces = spaces.get();
            let selected_space_index = spaces
                .iter()
                .position(|s| s.id == selected_space.id)
                .expect("selected space should present in loaded spaces");
            set_selected_space.call(
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
            set_selected_space.call(
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

    use_hotkeys!((key_bindings.select_prev_list_item) => move |_| select_prev_space());
    use_hotkeys!((key_bindings.select_next_list_item) => move |_| select_next_space());

    let global_config = expect_context::<RwSignal<Config>>();

    view! {
        <div class="spaces-scroll-area">
            {move || {
                let config = global_config.get();
                spaces.get().into_iter().map(|space| {
                    let selected = selected_space.get().as_ref().map(|selected| selected.id == space.id).unwrap_or_default();
                    view! { <Space space set_selected_space selected base_path=config.app.base_path.clone() minimized=spaces_minimized /> }
                }).collect_view()
            }}
        </div>
    }
}
