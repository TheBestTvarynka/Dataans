use common::note::File;
use tauri::State;
use uuid::Uuid;

use crate::dataans::{DataansError, DataansState};

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn upload_file(
    state: State<'_, DataansState>,
    id: Uuid,
    name: String,
    data: Vec<u8>,
) -> Result<File, DataansError> {
    state
        .file_service
        .upload_file(id, name, data, &state.app_data_dir)
        .await
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn delete_file(state: State<'_, DataansState>, id: Uuid) -> Result<(), DataansError> {
    state.file_service.delete_file(id).await
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn gen_random_avatar(state: State<'_, DataansState>) -> Result<File, DataansError> {
    state.file_service.gen_random_avatar(&state.app_data_dir).await
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn handle_clipboard_image(state: State<'_, DataansState>) -> Result<File, DataansError> {
    state.file_service.handle_clipboard_image(&state.app_data_dir).await
}
