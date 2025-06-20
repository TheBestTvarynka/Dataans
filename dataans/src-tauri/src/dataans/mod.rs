use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use common::APP_PLUGIN_NAME;
use sqlx::sqlite::SqlitePoolOptions;
use tauri::async_runtime::block_on;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::dataans::db::sqlite::SqliteDb;
use crate::dataans::db::OperationLogger;
use crate::{CONFIGS_DIR, CONFIG_FILE_NAME, FILES_DIR, IMAGES_DIR, PROFILE_DIR};

mod command;
mod crypto;
mod db;
pub mod error;
mod service;
mod sync;

use crate::dataans::error::DataansError;
use crate::dataans::service::file::FileService;
use crate::dataans::service::note::NoteService;
use crate::dataans::service::space::SpaceService;
use crate::dataans::service::web::WebService;

pub struct State<D> {
    app_data_dir: PathBuf,
    space_service: Arc<SpaceService<D>>,
    note_service: Arc<NoteService<D>>,
    file_service: Arc<FileService<D>>,
    web_service: Arc<WebService>,
    operation_logger: Arc<OperationLogger>,
}

pub type DataansState = State<SqliteDb>;

impl DataansState {
    pub async fn init(db_dir: PathBuf, app_data_dir: PathBuf) -> Self {
        // It's okay to panic in this function because the app is useless without a working db.

        let db_file = db_dir.join("dataans.sqlite");

        info!(?db_file, "Database file");

        if !db_file.exists() {
            std::fs::File::create(&db_file).expect("Can not create db file");
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect_lazy(&format!(
                "sqlite://{}",
                db_file.to_str().expect("Bro, wtf, use UTF-8 paths")
            ))
            .expect("can not connect to sqlite db");

        let operation_logger = Arc::new(OperationLogger::new(pool));
        let sqlite = Arc::new(SqliteDb::new(Arc::clone(&operation_logger)));

        let space_service = Arc::new(SpaceService::new(Arc::clone(&sqlite)));
        let note_service = Arc::new(NoteService::new(Arc::clone(&sqlite), Arc::clone(&space_service)));
        let file_service = Arc::new(FileService::new(Arc::clone(&sqlite)));
        let web_service =
            Arc::new(WebService::new(app_data_dir.join(PROFILE_DIR)).expect("can not initiate web service"));

        Self {
            app_data_dir,
            space_service,
            note_service,
            file_service,
            web_service,
            operation_logger,
        }
    }
}

pub fn init_dataans_plugin<R: Runtime>() -> TauriPlugin<R> {
    debug!("init_dataans_plugin");

    Builder::<R>::new(APP_PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            command::space::list_spaces,
            command::space::create_space,
            command::space::update_space,
            command::space::delete_space,
            command::note::list_notes,
            command::note::create_note,
            command::note::update_note,
            command::note::delete_note,
            command::note::search_notes_in_space,
            command::note::search_notes,
            command::file::upload_file,
            command::file::delete_file,
            command::file::gen_random_avatar,
            command::file::handle_clipboard_image,
            command::export::export_app_data,
            // command::import::import_app_data,
            command::web::sign_up,
            command::web::sign_in,
            command::web::profile,
            command::sync::set_sync_options,
            command::sync::full_sync,
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
            let images_dir = app_data.join(IMAGES_DIR);
            let configs_dir = app_data.join(CONFIGS_DIR);
            let profile_dir = app_data.join(PROFILE_DIR);

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

            if !profile_dir.exists() {
                match fs::create_dir(&profile_dir) {
                    Ok(()) => info!(?profile_dir, "Successfully created profile directory"),
                    Err(err) => error!(?err, ?profile_dir, "Filed to create profile directory"),
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

            let config = crate::config::read_config(config_file).expect("config reading should not fail");
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

            let dataans_state = block_on(DataansState::init(db_dir, app_data));
            if let Err(err) = block_on(dataans_state.file_service.check_default_space_avatar()) {
                error!(?err);
            }
            app_handle.manage(dataans_state);

            Ok(())
        })
        .build()
}
