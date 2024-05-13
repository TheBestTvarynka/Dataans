use leptos::*;

use crate::backend::load_theme;
use crate::messages::Messages;
use crate::profile::Profile;
use crate::spaces::Spaces;

#[component]
pub fn App() -> impl IntoView {
    let (theme_css, set_theme_css) = create_signal(String::default());

    spawn_local(async move {
        let theme = load_theme().await;
        set_theme_css.set(theme.to_css());
    });

    view! {
        <main class="app" style={move || theme_css.get()}>
            <Spaces />
            <Messages />
            <Profile />
        </main>
    }
}
