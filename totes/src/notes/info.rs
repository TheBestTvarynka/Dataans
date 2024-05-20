use common::space::Space;
use leptos::*;

#[component]
pub fn Info(current_space: Space<'static>) -> impl IntoView {
    view! {
        <div class="info">
            <span class="space-name">{String::from(current_space.name)}</span>
        </div>
    }
}
