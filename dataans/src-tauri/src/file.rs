use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::FILES_DIR;

#[instrument(ret, skip(app_handle, data))]
#[tauri::command]
pub fn upload_file(app_handle: AppHandle, id: Uuid, name: String, data: Vec<u8>) -> PathBuf {
    let file_name = format!("{}_{}", id, name);

    let file_path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_default()
        .join(FILES_DIR)
        .join(file_name);

    fs::write(&file_path, data).expect("Image data writing into the file should not fail");

    file_path
}

#[instrument]
#[tauri::command]
pub fn remove_file(path: PathBuf) {
    fs::remove_file(path).expect("File removing should not fail")
}
