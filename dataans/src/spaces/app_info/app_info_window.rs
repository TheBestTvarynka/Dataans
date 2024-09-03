use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

#[component]
pub fn AppInfoWindow(#[prop(into)] close: Callback<(), ()>) -> impl IntoView {
    use_hotkeys!(("Escape") => move |_| close.call(()));

    view! {
        <div class="app-into-window">
            <span class="app-into-window-title">{format!("Dataans v.{}", env!("CARGO_PKG_VERSION"))}</span>
            <span>"Take notes in the form of markdown snippets grouped into spaces."</span>
            <span>"Source code: "<a href="https://github.com/TheBestTvarynka/Dataans">"GitHub/TbeBestTvarynka/Dataans"</a></span>
            <hr style="width: 80%" />

        </div>
    }
}