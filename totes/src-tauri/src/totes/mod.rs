use std::fs;
use std::path::PathBuf;

use common::TOTES_PLUGIN_NAME;
use polodb_core::Database;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::{CONFIGS_DIR, FILES_DIR, IMAGED_DIR};

mod note;
mod space;

const SPACES_COLLECTION_NAME: &str = "spaces";
const NOTES_COLLECTION_NAME: &str = "notes";

pub struct TotesState {
    db: Database,
}

impl TotesState {
    pub fn init(db_dir: PathBuf) -> Self {
        let db_file = db_dir.join("totes.db");

        Self {
            db: Database::open_file(db_file).expect("Database opening should not fail."),
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
            note::search_note_in_space,
            note::search_note,
        ])
        .setup(|app_handle| {
            let app_data = app_handle.path_resolver().app_data_dir().unwrap_or_default();
            let db_dir = app_data.join("db");
            let files_dir = app_data.join(FILES_DIR);
            let images_dir = app_data.join(IMAGED_DIR);
            let configs_dir = app_data.join(CONFIGS_DIR);

            if !db_dir.exists() {
                match fs::create_dir(&db_dir) {
                    Ok(()) => info!("Successfully created totes database directory: {:?}", db_dir),
                    Err(err) => error!(
                        "Filed to create totes database directory: {:?}. Path: {:?}",
                        err, db_dir
                    ),
                }
            }

            if !files_dir.exists() {
                match fs::create_dir(&files_dir) {
                    Ok(()) => info!("Successfully created totes files directory: {:?}", files_dir),
                    Err(err) => error!(
                        "Filed to create totes files directory: {:?}. Path: {:?}",
                        err, files_dir
                    ),
                }
            }

            if !images_dir.exists() {
                match fs::create_dir(&images_dir) {
                    Ok(()) => info!("Successfully created totes images directory: {:?}", images_dir),
                    Err(err) => error!(
                        "Filed to create totes images directory: {:?}. Path: {:?}",
                        err, images_dir
                    ),
                }
            }

            if !configs_dir.exists() {
                match fs::create_dir(&configs_dir) {
                    Ok(()) => info!("Successfully created totes configs directory: {:?}", configs_dir),
                    Err(err) => error!(
                        "Filed to create totes configs directory: {:?}. Path: {:?}",
                        err, configs_dir
                    ),
                }
            }

            app_handle.manage(TotesState::init(db_dir));
            Ok(())
        })
        .build()
}
