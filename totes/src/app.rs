use leptos::*;

use crate::spaces::Spaces;
use crate::messages::Messages;
use crate::profile::Profile;

#[component]
pub fn App() -> impl IntoView {

    info!("App component");

    let theme = format!("--messages_background: {}", "green");

    view! {
        <main class="app" style=theme>
            <Spaces />
            <Messages />
            <Profile />
        </main>
    }
}
