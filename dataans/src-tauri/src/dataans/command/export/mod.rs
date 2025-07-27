mod json;
mod md;

use std::fs;
use std::path::{Path, PathBuf};

use common::DataExportConfig;
use common::error::CommandResult;
use tauri::State;
use time::OffsetDateTime;
use time::macros::format_description;

use crate::BACKUPS_DIR;
use crate::dataans::{DataansError, DataansState};

fn prepare_backups_dir(base_path: &Path) -> Result<PathBuf, DataansError> {
    let backups_dir = base_path.join(BACKUPS_DIR);

    if !backups_dir.exists() {
        match fs::create_dir(&backups_dir) {
            Ok(()) => info!(?backups_dir, "Successfully created backups directory"),
            Err(err) => {
                error!(?err, ?backups_dir, "Filed to create backups directory");
                Err(err)?;
            }
        }
    }

    let format = format_description!("[year].[month].[day]-[hour].[minute].[second]");
    let backups_dir = backups_dir.join(OffsetDateTime::now_utc().format(&format)?);

    fs::create_dir(&backups_dir)?;

    Ok(backups_dir)
}

async fn export_data(state: State<'_, DataansState>, export_config: DataExportConfig) -> Result<PathBuf, DataansError> {
    let backups_dir = prepare_backups_dir(&state.base_path)?;
    let spaces = state.space_service.spaces().await?;

    match export_config {
        DataExportConfig::Md(notes_export_option) => {
            md::export(&notes_export_option, &backups_dir, spaces, &state.note_service).await?
        }
        DataExportConfig::Json(schema_version) => {
            json::export(schema_version, &backups_dir, spaces, &state.note_service).await?
        }
    }

    Ok(backups_dir)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn export_app_data(
    state: State<'_, DataansState>,
    export_config: DataExportConfig,
) -> CommandResult<PathBuf> {
    Ok(export_data(state, export_config).await?)
}
