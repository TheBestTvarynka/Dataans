use common::note::File;
use tauri::State;
use uuid::Uuid;

use super::CommandResult;
use crate::dataans::DataansState;

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn upload_file(state: State<'_, DataansState>, id: Uuid, name: String, data: Vec<u8>) -> CommandResult<File> {
    Ok(state
        .file_service
        .upload_file(id, name, &data, &state.app_data_dir)
        .await
        .into())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn delete_file(state: State<'_, DataansState>, id: Uuid) -> CommandResult<()> {
    Ok(state.file_service.delete_file(id).await.into())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn gen_random_avatar(state: State<'_, DataansState>) -> CommandResult<File> {
    Ok(state.file_service.gen_random_avatar(&state.app_data_dir).await.into())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn handle_clipboard_image(state: State<'_, DataansState>) -> CommandResult<File> {
    Ok(state
        .file_service
        .handle_clipboard_image(&state.app_data_dir)
        .await
        .into())
}
