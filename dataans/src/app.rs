use common::note::Note;
use common::profile::UserContext;
use common::space::{Id as SpaceId, OwnedSpace, sort_spaces};
use common::{Config, CreationDate};
use leptoaster::*;
use leptos::context::provide_context;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::*;
use leptos_router::path;

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
    /// Loaded spaces.
    ///
    /// INVARIANT: Spaces are sorted by [sort_spaces].
    /// Change spaces only through [GlobalState] methods, which keep the invariant.
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

impl GlobalState {
    /// Replaces all spaces.
    pub fn set_spaces(&mut self, mut spaces: Vec<OwnedSpace>) {
        sort_spaces(&mut spaces);
        self.spaces = spaces;
    }

    /// Adds a new space or replaces the existing one with the same id.
    pub fn add_space(&mut self, space: OwnedSpace) {
        self.spaces.retain(|s| s.id != space.id);
        self.spaces.push(space);
        sort_spaces(&mut self.spaces);
    }

    /// Replaces the existing space with the same id.
    ///
    /// Returns `false` if the space does not exist.
    pub fn update_space(&mut self, space: OwnedSpace) -> bool {
        let Some(local_space) = self.spaces.iter_mut().find(|s| s.id == space.id) else {
            return false;
        };
        *local_space = space;
        sort_spaces(&mut self.spaces);

        true
    }

    /// Removes the space.
    pub fn remove_space(&mut self, space_id: SpaceId) {
        // Removal keeps the remaining spaces sorted.
        self.spaces.retain(|s| s.id != space_id);
    }

    /// Records that a note created at `note_created_at` has been added to the space.
    pub fn note_added(&mut self, space_id: SpaceId, note_created_at: &CreationDate) {
        if let Some(space) = self.spaces.iter_mut().find(|s| s.id == space_id) {
            space.note_added(note_created_at);
            sort_spaces(&mut self.spaces);
        }
    }

    /// Records that a space note has been deleted.
    ///
    /// `newest_note_created_at` is the creation date of the newest remaining space note, or [None] if no notes are left.
    pub fn note_deleted(&mut self, space_id: SpaceId, newest_note_created_at: Option<&CreationDate>) {
        if let Some(space) = self.spaces.iter_mut().find(|s| s.id == space_id) {
            space.note_deleted(newest_note_created_at);
            sort_spaces(&mut self.spaces);
        }
    }

    /// Sets the space [newest note](common::space::Space::newest_note) date computed by the backend.
    pub fn set_space_newest_note(&mut self, space_id: SpaceId, newest_note: CreationDate) {
        if let Some(space) = self.spaces.iter_mut().find(|s| s.id == space_id) {
            space.newest_note = newest_note;
            sort_spaces(&mut self.spaces);
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(RwSignal::new(GlobalState::default()));
    provide_context(RwSignal::new(Config::default()));
    provide_context(RwSignal::new(Option::<UserContext>::None));
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

    let (theme_css, set_theme_css) = signal(String::default());

    let global_config = expect_context::<RwSignal<Config>>();
    spawn_local(async move {
        let config = try_exec!(load_config().await, "Failed to load config", toaster);
        info!(?config, "Loaded config:");
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
        |state, spaces| state.set_spaces(spaces),
    );

    view! {
        <Router>
            <Toaster stacked=true />
            <main class="app" style=move || theme_css.get()>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=move || view! {
                        <Spaces spaces set_spaces />
                        <Notes />
                    } />
                    <Route path=path!("/app-info") view=AppInfo />
                </Routes>
            </main>
        </Router>
    }
}
