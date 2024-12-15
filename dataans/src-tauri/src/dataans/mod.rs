use std::fs;
use std::path::PathBuf;

use common::APP_PLUGIN_NAME;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::{CONFIGS_DIR, CONFIG_FILE_NAME, FILES_DIR, IMAGED_DIR};

mod db;
mod export;
mod note;
mod space;

pub struct DataansState {
    app_data_dir: PathBuf,
    db: SqlitePool,
}

impl DataansState {
    pub async fn init(db_dir: PathBuf, app_data_dir: PathBuf) -> Self {
        // It's okay to panic in this function because the app is useless without a working db.

        let db_file = db_dir.join("dataans.sqlite");

        info!(?db_file, "Database file");

        if !db_file.exists() {
            std::fs::File::create(&db_file).expect("Can not create db file");
        }

        let db = SqlitePoolOptions::new()
            .max_connections(4)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect_lazy(&format!(
                "sqlite://{}",
                db_file.to_str().expect("Bro, wtf, use UTF-8 paths")
            ))
            .expect("can not connect to sqlite db");

        Self { app_data_dir, db }
    }
}

pub fn init_dataans_plugin<R: Runtime>() -> TauriPlugin<R> {
    debug!("init_dataans_plugin");

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
            export::export_app_data,
        ])
        .setup(|app_handle, _api| {
            info!("Starting app setup...");

            let path_resolver = app_handle.path();
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
                let resource_dir = path_resolver.resource_dir()?.join("resources");
                let default_config = resource_dir.join("configs").join("config.toml");

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
            }

            let config = crate::config::read_config(config_file);
            let theme_file = configs_dir.join(&config.appearance.theme);
            if !theme_file.exists() {
                let resource_dir = path_resolver.resource_dir()?.join("resources");
                let default_theme = resource_dir.join("configs").join("theme_dark.toml");

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
            }

            app_handle.manage(DataansState::init(db_dir, app_data));

            Ok(())
        })
        .build()
}
