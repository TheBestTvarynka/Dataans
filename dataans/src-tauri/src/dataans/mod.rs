use std::fs;
use std::path::PathBuf;

use common::APP_PLUGIN_NAME;
use polodb_core::Database;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::{CONFIGS_DIR, FILES_DIR, IMAGED_DIR};

mod note;
mod space;

const SPACES_COLLECTION_NAME: &str = "spaces";
const NOTES_COLLECTION_NAME: &str = "notes";

pub struct DataansState {
    db: Database,
}

impl DataansState {
    pub fn init(db_dir: PathBuf) -> Self {
        let db_file = db_dir.join("dataans.db");

        info!(?db_file, "database file");

        Self {
            db: Database::open_file(db_file).expect("Database opening should not fail."),
        }
    }
}

pub fn init_dataans_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(APP_PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            space::list_spaces,
            space::create_space,
            space::update_space,
            space::delete_space,
            note::list_notes,
            note::create_note,
            note::update_note,
            note::delete_note,
            note::search_notes_in_space,
            note::search_notes,
        ])
        .setup(|app_handle| {
            info!("Starting app setup...");

            let app_data = app_handle.path_resolver().app_data_dir().unwrap_or_default();
            debug!(?app_data);
            if !app_data.exists() {
                match fs::create_dir(&app_data) {
                    Ok(()) => info!("Successfully created app data directory: {:?}", app_data),
                    Err(err) => error!("Filed to create app data directory: {:?}. Path: {:?}", err, app_data),
                }
            }

            let db_dir = app_data.join("db");
            let files_dir = app_data.join(FILES_DIR);
            let images_dir = app_data.join(IMAGED_DIR);
            let configs_dir = app_data.join(CONFIGS_DIR);

            if !db_dir.exists() {
                match fs::create_dir(&db_dir) {
                    Ok(()) => info!("Successfully created database directory: {:?}", db_dir),
                    Err(err) => error!("Filed to create database directory: {:?}. Path: {:?}", err, db_dir),
                }
            }

            if !files_dir.exists() {
                match fs::create_dir(&files_dir) {
                    Ok(()) => info!("Successfully created files directory: {:?}", files_dir),
                    Err(err) => error!("Filed to create files directory: {:?}. Path: {:?}", err, files_dir),
                }
            }

            if !images_dir.exists() {
                match fs::create_dir(&images_dir) {
                    Ok(()) => info!("Successfully created images directory: {:?}", images_dir),
                    Err(err) => error!("Filed to create images directory: {:?}. Path: {:?}", err, images_dir),
                }
            }

            // TODO: initialize default configs if they are not exist.
            if !configs_dir.exists() {
                match fs::create_dir(&configs_dir) {
                    Ok(()) => info!("Successfully created configs directory: {:?}", configs_dir),
                    Err(err) => error!("Filed to create configs directory: {:?}. Path: {:?}", err, configs_dir),
                }
            }

            app_handle.manage(DataansState::init(db_dir));
            Ok(())
        })
        .build()
}
