use std::path::PathBuf;

use arboard::Clipboard;
use image::{ImageBuffer, Rgba};
use tauri::AppHandle;
use uuid::Uuid;

use crate::IMAGED_DIR;

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

#[tauri::command]
pub fn handle_clipboard_image(app_handle: AppHandle) -> PathBuf {
    let mut clipboard = Clipboard::new().expect("Initialized Clipboard object");
    let image_data = clipboard.get_image().expect("Image data");

    let image_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_default()
        .join(IMAGED_DIR)
        .join(format!("{}.png", Uuid::new_v4()));

    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
        image_data.width.try_into().unwrap(),
        image_data.height.try_into().unwrap(),
        image_data.bytes.as_ref(),
    )
    .expect("ImageBuffer object");
    img.save(&image_path).expect("Clipboard image saving should not fail");

    image_path
}
