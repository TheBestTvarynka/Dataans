mod json;

use crate::dataans::{DataansError, DataansState};
use common::error::CommandResult;
use std::path::PathBuf;
use tauri::State;

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn import_app_data(state: State<'_, DataansState>, path: PathBuf) -> CommandResult<()> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    if extension == "json" {
        info!(?path, "Processing JSON import...");
        json::import(&path, &state.space_service, &state.note_service).await?;

        Ok(())
    } else {
        error!(extension, "Unsupported file type");
        Err(DataansError::IncorrectImportFileType(extension.to_string()).into())
    }
}
