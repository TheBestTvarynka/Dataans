use std::path::PathBuf;

use tauri::AppHandle;

use crate::IMAGES_FOLDER;

#[tauri::command]
pub fn image_path(app_handle: AppHandle, image_name: String) -> PathBuf {
    let image_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(IMAGES_FOLDER)
        .join(image_name);
    info!("Image path: {:?}", image_path);

    image_path
}
