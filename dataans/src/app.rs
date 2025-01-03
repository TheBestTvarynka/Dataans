use common::note::Note;
use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};

use crate::backend::{load_config, load_theme};
use crate::notes::Notes;
use crate::spaces::Spaces;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum FindNoteMode {
    #[default]
    None,
    FindNote {
        space: Option<OwnedSpace>,
    },
}

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub spaces: Vec<OwnedSpace>,
    pub notes: Vec<Note<'static>>,
    pub selected_space: Option<OwnedSpace>,
    pub minimize_spaces: bool,
    pub find_note_mode: FindNoteMode,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            spaces: Default::default(),
            notes: Default::default(),
            selected_space: Default::default(),
            minimize_spaces: true,
            find_note_mode: Default::default(),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(GlobalState::default()));
    provide_context(create_rw_signal(Config::default()));

    let (theme_css, set_theme_css) = create_signal(String::default());
    let (config, set_config) = create_signal(Config::default());

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    let global_config = expect_context::<RwSignal<Config>>();
    spawn_local(async move {
        match load_config().await {
            Ok(config) => {
                info!("{:?}", config);

                let theme = config.appearance.theme.clone();

                global_config.set(config.clone());
                set_config.set(config);

                // TODO.
                let theme = load_theme(&theme).await.unwrap();
                set_theme_css.set(theme.to_css());
            }
            Err(err) => {
                error!("{:?}", err);
                // TODO: toastr.
            }
        }
    });

    let global_state = expect_context::<RwSignal<GlobalState>>();

    let (spaces, set_spaces) = create_slice(
        global_state,
        |state| state.spaces.clone(),
        |state, spaces| state.spaces = spaces,
    );

    view! {
        <main class="app" style=move || theme_css.get() _ref=main_ref>
            {move || view! { <Spaces config=config.get() spaces set_spaces /> }}
            {move || view! { <Notes config=config.get() /> }}
        </main>
    }
}
