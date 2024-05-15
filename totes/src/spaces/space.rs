use common::space::Space as SpaceData;
use leptos::*;

#[component]
pub fn Space<'name>(space: SpaceData<'name>) -> impl IntoView {
    view! {
        <div class="space">
            // TODO(@TheBestTvarynka): implement space avatar image.
            <img class="space-avatar" alt="space avatar image" src="https://avatars.githubusercontent.com/u/43034350?v=4" />
            <span class="space-title">{space.name.as_ref().to_string()}</span>
        </div>
    }
}
