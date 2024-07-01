use common::note::Note;
use common::space::OwnedSpace;
use leptos::*;

use crate::backend::load_theme;
use crate::notes::Notes;
// use crate::profile::Profile;
use crate::spaces::Spaces;

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub spaces: Vec<OwnedSpace>,
    pub notes: Vec<Note<'static>>,
    pub selected_space: Option<OwnedSpace>,
}

#[component]
pub fn App() -> impl IntoView {
    let (theme_css, set_theme_css) = create_signal(String::default());

    spawn_local(async move {
        let theme = load_theme().await;
        set_theme_css.set(theme.to_css());
    });

    provide_context(create_rw_signal(GlobalState::default()));

    view! {
        <main class="app" style=move || theme_css.get()>
            <Spaces />
            <Notes />
            // <Profile />
        </main>
    }
}
