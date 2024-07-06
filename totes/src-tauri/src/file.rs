use std::fs;
use std::path::PathBuf;

use tauri::AppHandle;
use uuid::Uuid;

use crate::FILES_FOLDER;

#[tauri::command]
pub fn upload_file(app_handle: AppHandle, id: Uuid, name: String, data: Vec<u8>) -> PathBuf {
    let file_name = format!("{}_{}", name, id);

    let file_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(FILES_FOLDER)
        .join(file_name);

    fs::write(&file_path, data).expect("Image data writing into the file should not fail");

    file_path
}

#[tauri::command]
pub fn remove_file(path: PathBuf) {
    fs::remove_file(path).expect("File removing should not fail")
}
