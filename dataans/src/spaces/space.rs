use common::space::OwnedSpace;
use leptos::prelude::*;

use crate::backend::convert_file_src;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Space(
    space: OwnedSpace,
    #[prop(into)] set_selected_space: Callback<OwnedSpace, ()>,
    selected: bool,
    base_path: String,
    minimized: Signal<bool>,
) -> impl IntoView {
    let class = if selected {
        "selected-space space"
    } else {
        "simple-space space"
    };

    let space_data = space.clone();
    let space_name = move || space_data.name.to_string();

    let space_data = space.clone();

    view! {
        <div class=class on:click=move |_| set_selected_space.call(space_data.clone()) title=space_name>
            <img class="space-avatar" alt="space avatar image" src=convert_file_src(space.avatar.path(), &base_path) />
            <Show when=move || !minimized.get()>
                <span class="space-title">{space.name.to_string()}</span>
            </Show>
        </div>
    }
}
