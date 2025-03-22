mod json;

use crate::dataans::{DataansError, DataansState};
use common::error::CommandResult;
use std::path::PathBuf;
use tauri::State;

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn import_app_data(state: State<'_, DataansState>, path: PathBuf) -> CommandResult<()> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    match extension.as_str() {
        "json" => {
            info!("Processing JSON import from {:?}", path);
            json::import(&path, &state.space_service, &state.note_service)
                .await
                .map_err(|e| e.into())
        }
        _ => {
            error!("Unsupported file type: {:?}", extension);
            Err(DataansError::IncorrectFileType(extension.to_string()).into())
        }
    }
}
