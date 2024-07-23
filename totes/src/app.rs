use common::note::Note;
use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};

use crate::backend::{load_config, load_theme};
use crate::notes::Notes;
// use crate::profile::Profile;
use crate::spaces::Spaces;

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub spaces: Vec<OwnedSpace>,
    pub notes: Vec<Note<'static>>,
    pub selected_space: Option<OwnedSpace>,
    pub minimize_spaces: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            spaces: Default::default(),
            notes: Default::default(),
            selected_space: Default::default(),
            minimize_spaces: true,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(Config::default()));
    provide_context(create_rw_signal(GlobalState::default()));

    let (theme_css, set_theme_css) = create_signal(String::default());

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    let config = expect_context::<RwSignal<Config>>();
    let (_, set_config) = create_slice(config, |_config| (), |config, new_config| *config = new_config);

    spawn_local(async move {
        let config = load_config().await;
        let theme = config.appearance.theme.clone();
        info!("config: {:?}", config);
        set_config.set(config);

        let theme = load_theme(&theme).await;
        set_theme_css.set(theme.to_css());
    });

    view! {
        <main class="app" style=move || theme_css.get() _ref=main_ref>
            <Spaces />
            <Notes />
            // <Profile />
        </main>
    }
}
