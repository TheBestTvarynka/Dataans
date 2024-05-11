pub mod editor;
mod info;

use leptos::*;

use self::editor::Editor;
use self::info::Info;

#[component]
pub fn Messages() -> impl IntoView {
    view! {
        <div class="messages">
            <Info />
            <span>"Messages"</span>
            <Editor />
        </div>
    }
}
