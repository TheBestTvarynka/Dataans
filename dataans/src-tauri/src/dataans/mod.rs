use std::fs;
use std::path::PathBuf;

use common::APP_PLUGIN_NAME;
use polodb_core::Database;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::{CONFIGS_DIR, CONFIG_FILE_NAME, FILES_DIR, IMAGED_DIR};

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

            let path_resolver = app_handle.path_resolver();
            let app_data = path_resolver.app_data_dir().unwrap_or_default();
            debug!(?app_data);
            if !app_data.exists() {
                match fs::create_dir(&app_data) {
                    Ok(()) => info!(?app_data, "Successfully created app data directory"),
                    Err(err) => error!(?err, ?app_data, "Filed to create app data directory"),
                }
            }

            let db_dir = app_data.join("db");
            let files_dir = app_data.join(FILES_DIR);
            let images_dir = app_data.join(IMAGED_DIR);
            let configs_dir = app_data.join(CONFIGS_DIR);

            if !db_dir.exists() {
                match fs::create_dir(&db_dir) {
                    Ok(()) => info!(?db_dir, "Successfully created database directory"),
                    Err(err) => error!(?err, ?db_dir, "Filed to create database directory"),
                }
            }

            if !files_dir.exists() {
                match fs::create_dir(&files_dir) {
                    Ok(()) => info!(?files_dir, "Successfully created files directory"),
                    Err(err) => error!(?err, ?files_dir, "Filed to create files directory"),
                }
            }

            if !images_dir.exists() {
                match fs::create_dir(&images_dir) {
                    Ok(()) => info!(?images_dir, "Successfully created images directory"),
                    Err(err) => error!(?err, ?images_dir, "Filed to create images directory"),
                }
            }

            if !configs_dir.exists() {
                match fs::create_dir(&configs_dir) {
                    Ok(()) => info!(?configs_dir, "Successfully created configs directory"),
                    Err(err) => error!(?err, ?configs_dir, "Filed to create configs directory"),
                }
            }

            let config_file = configs_dir.join(CONFIG_FILE_NAME);
            if !config_file.exists() {
                if let Some(default_config) = path_resolver.resolve_resource("resources/configs/config.toml") {
                    if let Err(err) = fs::copy(&default_config, &config_file) {
                        error!(
                            ?err,
                            ?default_config,
                            ?config_file,
                            "Cannot create the default config file"
                        );
                    } else {
                        info!(?config_file, "Successfully created default config file");
                    }
                } else {
                    error!(
                        "Cannot to resolve the default config file. You need to fix it manually or reinstall the app"
                    );
                }
            }

            let config = crate::config::read_config(config_file);
            let theme_file = configs_dir.join(&config.appearance.theme);
            if !theme_file.exists() {
                if let Some(default_theme) = path_resolver.resolve_resource("resources/configs/theme_dark.toml") {
                    if let Err(err) = fs::copy(&default_theme, &theme_file) {
                        error!(
                            ?err,
                            ?default_theme,
                            ?theme_file,
                            "Cannot create the default theme file"
                        );
                    } else {
                        info!(?theme_file, "Successfully created default theme file");
                    }
                } else {
                    error!(
                        "Cannot to resolve the default theme file. You need to fix it manually or reinstall the app"
                    );
                }
            }

            app_handle.manage(DataansState::init(db_dir));

            Ok(())
        })
        .build()
}
