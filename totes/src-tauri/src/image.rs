use std::fs;
use std::path::PathBuf;

use tauri::AppHandle;
use uuid::Uuid;

use crate::IMAGED_DIR;

#[tauri::command]
pub fn save_image(app_handle: AppHandle, image_name: String, image_data: Vec<u8>) -> PathBuf {
    let image_name = format!("{}_{}", Uuid::new_v4(), image_name);

    let image_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(IMAGED_DIR)
        .join(image_name);

    fs::write(&image_path, image_data).expect("Image data writing into the file should not fail");

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
        .join(IMAGED_DIR)
        .join(avatar_name);
    avatar.save(&avatar_path).expect("Avatar image saving should not fail");

    info!("Avatar image path: {:?}", avatar_path);

    avatar_path
}
