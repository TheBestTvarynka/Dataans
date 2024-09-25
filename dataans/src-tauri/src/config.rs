use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use common::{Config, Theme};
use tauri::AppHandle;

use super::{CONFIGS_DIR, CONFIG_FILE_NAME};

pub fn read_config(path: impl AsRef<Path>) -> Config {
    let config_file_path = path.as_ref();
    let config_data = match read_to_string(config_file_path) {
        Ok(data) => data,
        Err(err) => {
            error!(?err, ?config_file_path, "Can not read config file",);
            return Default::default();
        }
    };

    toml::from_str(&config_data).unwrap_or_else(|err| {
        error!(?err, "Can not parse config");
        Default::default()
    })
}

pub fn load_config_inner(app_handle: &AppHandle) -> Config {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);
    info!(?config_file_path, "Config file path");

    read_config(config_file_path)
}

#[instrument(ret, skip(app_handle))]
#[tauri::command]
pub fn config(app_handle: AppHandle) -> Config {
    load_config_inner(&app_handle)
}

#[instrument(level = "trace", ret, skip(app_handle))]
#[tauri::command]
pub fn theme(app_handle: AppHandle, file_path: PathBuf) -> Theme {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let theme_file_path = configs_dir.join(file_path);
    info!(?theme_file_path, "Theme file path");

    let theme_data = match read_to_string(&theme_file_path) {
        Ok(data) => data,
        Err(err) => {
            error!(?err, ?theme_file_path, "Can not read theme config file",);
            return Default::default();
        }
    };

    toml::from_str(&theme_data).unwrap_or_else(|err| {
        error!(?err, "Can not parse theme config");
        Default::default()
    })
}

#[tauri::command]
pub fn open_config_file(app_handle: AppHandle) {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);

    info!(open_config_file_result = ?opener::open(&config_file_path));
}

#[tauri::command]
pub fn open_theme_file(app_handle: AppHandle, file_path: PathBuf) {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let theme_file_path = configs_dir.join(file_path);

    info!(open_config_file_result = ?opener::reveal(&theme_file_path));
}

#[tauri::command]
pub fn open_config_file_folder(app_handle: AppHandle) {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let config_file_path = configs_dir.join(CONFIG_FILE_NAME);

    info!(open_config_file_folder_result = ?opener::reveal(&config_file_path));
}

#[instrument]
#[tauri::command]
pub fn reveal(path: PathBuf) {
    info!(reveal_note_file_result = ?opener::reveal(&path));
}

#[instrument]
#[tauri::command]
pub fn open(path: PathBuf) {
    info!(open_note_file_result = ?opener::open(&path));
}
