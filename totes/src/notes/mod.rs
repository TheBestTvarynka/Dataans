pub mod editor;
mod info;

use leptos::*;

use self::editor::Editor;
use self::info::Info;

#[component]
pub fn Notes() -> impl IntoView {
    view! {
        <div class="notes">
            <Info />
            <div class="notes-inner">
                <span>"Messages"</span>
                <Editor />
            </div>
        </div>
    }
}
