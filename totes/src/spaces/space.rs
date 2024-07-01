use common::space::OwnedSpace;
use leptos::*;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Space(
    space: OwnedSpace,
    #[prop(into)] set_selected_space: Callback<OwnedSpace, ()>,
    selected: bool,
) -> impl IntoView {
    let class = if selected {
        "selected-space space"
    } else {
        "simple-space space"
    };

    let space_data = space.clone();

    view! {
        <div class=class on:click=move |_| set_selected_space.call(space_data.clone())>
            <img class="space-avatar" alt="space avatar image" src=space.avatar.to_string() />
            <span class="space-title">{space.name.as_ref().to_string()}</span>
        </div>
    }
}
