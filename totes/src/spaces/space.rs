use common::space::Space as SpaceData;
use leptos::*;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Space(
    space: SpaceData<'static>,
    #[prop(into)] set_selected_space: Callback<SpaceData<'static>, ()>,
    selected: bool,
) -> impl IntoView {
    let class = if selected {
        "selected-space space"
    } else {
        "simple-space space"
    };

    let space_data = space.clone();

    view! {
        <div class={class} on:click=move |_| set_selected_space.call(space_data.clone())>
            // TODO(@TheBestTvarynka): implement space avatar image.
            <img class="space-avatar" alt="space avatar image" src="https://avatars.githubusercontent.com/u/43034350?v=4" />
            <span class="space-title">{space.name.as_ref().to_string()}</span>
        </div>
    }
}
