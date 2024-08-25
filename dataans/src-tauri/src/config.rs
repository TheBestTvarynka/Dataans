use std::fs::read_to_string;
use std::path::PathBuf;

use common::{Config, Theme};
use tauri::AppHandle;

use super::CONFIGS_DIR;

pub fn load_config_inner(app_handle: &AppHandle) -> Config {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let config_file_path = configs_dir.join("config.toml");
    info!("Config file path: {:?}", config_file_path);

    let config_data = match read_to_string(&config_file_path) {
        Ok(data) => data,
        Err(err) => {
            error!(
                "Can not read config file: {:?}. Filepath: `{:?}`.",
                err, config_file_path
            );
            return Default::default();
        }
    };

    toml::from_str(&config_data).unwrap_or_else(|err| {
        error!("Can not parse config: {:?}", err);
        Default::default()
    })
}

#[tauri::command]
pub fn config(app_handle: AppHandle) -> Config {
    load_config_inner(&app_handle)
}

#[tauri::command]
pub fn theme(app_handle: AppHandle, file_path: PathBuf) -> Theme {
    let configs_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(CONFIGS_DIR);

    let theme_file_path = configs_dir.join(file_path);
    info!("Theme file path: {:?}", theme_file_path);

    let theme_data = match read_to_string(&theme_file_path) {
        Ok(data) => data,
        Err(err) => {
            error!(
                "Can not read theme config file: {:?}. Filepath: `{:?}`.",
                err, theme_file_path
            );
            return Default::default();
        }
    };

    toml::from_str(&theme_data).unwrap_or_else(|err| {
        error!("Can not parse theme config: {:?}", err);
        Default::default()
    })
}

#[tauri::command]
pub fn reveal(path: PathBuf) {
    info!("Revealing the file: {:?}", path);

    info!("{:?}", opener::reveal(&path));
}

#[tauri::command]
pub fn open(path: PathBuf) {
    info!("Opening the file: {:?}", path);

    info!("{:?}", opener::open(&path));
}
