use common::TOTES_PLUGIN_NAME;
use polodb_core::Database;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

mod note;
mod space;

const DATABASE_FILEPATH: &str = "../db/totes.db";
const SPACES_COLLECTION_NAME: &str = "spaces";
const NOTES_COLLECTION_NAME: &str = "notes";

pub struct TotesState {
    db: Database,
}

impl TotesState {
    pub fn init() -> Self {
        // TODO(@TheBestTvarynka): implement database filepath detection.
        Self {
            db: Database::open_file(DATABASE_FILEPATH).expect("Database opening should not fail."),
        }
    }
}

pub fn init_totes_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(TOTES_PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            space::list_spaces,
            space::create_space,
            space::update_space,
            space::delete_space,
            note::list_notes,
            note::create_note,
            note::update_note,
            note::delete_note,
        ])
        .setup(|app_handle| {
            app_handle.manage(TotesState::init());
            Ok(())
        })
        .build()
}
