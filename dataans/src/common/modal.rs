use leptos::{component, view, Children, IntoView};

#[component]
pub fn Modal(children: Children) -> impl IntoView {
    view! {
        <div class="modal">
            {children()}
        </div>
    }
}
