use leptos::children::Children;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Modal(children: Children) -> impl IntoView {
    view! {
        <div class="modal">
            {children()}
        </div>
    }
}
