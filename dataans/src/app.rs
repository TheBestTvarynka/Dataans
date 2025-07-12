use common::note::Note;
use common::profile::UserContext;
use common::space::OwnedSpace;
use common::Config;
use leptoaster::*;
use leptos::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};
use leptos_router::{Route, Router, Routes};

use crate::app_info::AppInfo;
use crate::backend::auth::profile;
use crate::backend::sync::{on_data, on_status_update, on_user_context};
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
    provide_context(create_rw_signal(Option::<UserContext>::None));
    provide_toaster();

    let toaster = leptoaster::expect_toaster();

    let user_context = expect_context::<RwSignal<Option<UserContext>>>();
    let t = toaster.clone();
    spawn_local(async move {
        try_exec!(
            on_user_context(|data| user_context.set(data)).await,
            "Failed to listen on user context events",
            t
        );
    });

    let app_data = expect_context::<RwSignal<GlobalState>>();
    let t = toaster.clone();
    spawn_local(async move {
        try_exec!(on_data(app_data).await, "Failed to listen on data events", t);
    });

    let t = toaster.clone();
    spawn_local(async move {
        let toaster = t.clone();
        try_exec!(
            on_status_update(toaster).await,
            "Failed to listen on status update events",
            t
        );
    });

    let (theme_css, set_theme_css) = create_signal(String::default());

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    let global_config = expect_context::<RwSignal<Config>>();
    spawn_local(async move {
        let config = try_exec!(load_config().await, "Failed to load config", toaster);
        info!("Loaded config: {config:?}");
        let theme = config.appearance.theme.clone();

        global_config.set(config.clone());

        set_theme_css.set(try_exec!(load_theme(&theme).await, "Failed to load theme", toaster).to_css());

        if let Some(context) = try_exec!(profile().await, "Failed to load user profile", toaster) {
            user_context.set(Some(context));
        }
    });

    let global_state = expect_context::<RwSignal<GlobalState>>();

    let (spaces, set_spaces) = create_slice(
        global_state,
        |state| state.spaces.clone(),
        |state, spaces| state.spaces = spaces,
    );

    view! {
        <Router>
            <Toaster stacked=true />
            <main class="app" style=move || theme_css.get() _ref=main_ref>
                <Routes>
                    <Route path="/" view=move || view! {
                        <Spaces spaces set_spaces />
                        <Notes />
                    } />
                    <Route path="/app-info" view=AppInfo />
                </Routes>
            </main>
        </Router>
    }
}
