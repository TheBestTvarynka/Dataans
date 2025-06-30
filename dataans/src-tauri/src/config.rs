use std::fs::read_to_string;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};

use common::error::CommandResult;
use common::{Config, Theme};
use tauri::{AppHandle, Manager, Runtime};

use super::{CONFIGS_DIR, CONFIG_FILE_NAME, FILES_DIR};

pub fn read_config(path: impl AsRef<Path>) -> Result<Config, IoError> {
    let config_file_path = path.as_ref();
    let config_data = read_to_string(config_file_path)?;

    toml::from_str(&config_data).map_err(|err| IoError::new(IoErrorKind::InvalidInput, err))
}

pub fn load_config_inner<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Config, IoError> {
    let configs_dir = app_handle.path().app_data_dir().unwrap_or_default().join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);
    info!(?config_file_path, "Config file path");

    let mut config = read_config(config_file_path)?;

    if config.app.base_path.is_empty() {
        config.app.base_path = app_handle
            .path()
            .app_data_dir()
            .unwrap_or_default()
            .to_str()
            .expect("Bro, wtf, use UTF-8 paths")
            .to_owned();
    }

    Ok(config)
}

#[instrument(ret, skip(app_handle))]
#[tauri::command]
pub fn config<R: Runtime>(app_handle: AppHandle<R>) -> CommandResult<Config> {
    Ok(load_config_inner(&app_handle)?)
}

#[instrument(level = "trace", ret, skip(app_handle))]
#[tauri::command]
pub fn theme(app_handle: AppHandle, file_path: PathBuf) -> CommandResult<Theme> {
    let configs_dir = app_handle.path().app_data_dir().unwrap_or_default().join(CONFIGS_DIR);

    let theme_file_path = configs_dir.join(file_path);
    info!(?theme_file_path, "Theme file path");

    let theme_data = read_to_string(&theme_file_path)?;

    Ok(toml::from_str(&theme_data).map_err(|err| IoError::new(IoErrorKind::InvalidInput, err))?)
}

#[tauri::command]
pub fn open_config_file(app_handle: AppHandle) {
    let configs_dir = app_handle.path().app_data_dir().unwrap_or_default().join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);

    let open_config_file_result = opener::open(&config_file_path);
    info!(?open_config_file_result);
}

#[tauri::command]
pub fn open_theme_file(app_handle: AppHandle, file_path: PathBuf) {
    let configs_dir = app_handle.path().app_data_dir().unwrap_or_default().join(CONFIGS_DIR);

    let theme_file_path = configs_dir.join(file_path);

    let open_theme_file_result = opener::open(&theme_file_path);
    info!(?open_theme_file_result);
}

#[tauri::command]
pub fn open_config_file_folder(app_handle: AppHandle) {
    let configs_dir = app_handle.path().app_data_dir().unwrap_or_default().join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);

    let open_config_file_folder_result = opener::reveal(&config_file_path);
    info!(?open_config_file_folder_result);
}

#[instrument]
#[tauri::command]
pub fn reveal(app_handle: AppHandle, path: PathBuf) {
    let path_resolver = app_handle.path();
    let files_dir = path_resolver.app_data_dir().unwrap_or_default().join(FILES_DIR);
    let file = files_dir.join(path);

    let reveal_note_file_result = opener::reveal(&file);
    info!(?reveal_note_file_result);
}

#[instrument]
#[tauri::command]
pub fn open(app_handle: AppHandle, path: PathBuf) {
    let path_resolver = app_handle.path();
    let files_dir = path_resolver.app_data_dir().unwrap_or_default().join(FILES_DIR);
    let file = files_dir.join(path);

    let open_note_file_result = opener::open(&file);
    info!(?open_note_file_result);
}
