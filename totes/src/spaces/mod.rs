mod tools;

use leptos::*;

use self::tools::Tools;

#[component]
pub fn Spaces() -> impl IntoView {
    view! {
        <div class="spaces">
            <Tools />
            <span>"Spaces"</span>
        </div>
    }
}
