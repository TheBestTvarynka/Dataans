use std::path::PathBuf;

use tauri::AppHandle;
use uuid::Uuid;

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

#[tauri::command]
pub fn gen_random_avatar(app_handle: AppHandle) -> PathBuf {
    let avatar = avatar_generator::generate::avatar();
    let avatar_name = format!("{}.png", Uuid::new_v4());

    let avatar_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(IMAGES_FOLDER)
        .join(avatar_name);
    avatar.save(&avatar_path).expect("Avatar image saving should not fail");

    info!("Avatar image path: {:?}", avatar_path);

    avatar_path
}
