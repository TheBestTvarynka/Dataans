use leptos::*;

use crate::spaces::Spaces;
use crate::messages::Messages;
use crate::profile::Profile;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app">
            <Spaces />
            <Messages />
            <Profile />
        </main>
    }
}
