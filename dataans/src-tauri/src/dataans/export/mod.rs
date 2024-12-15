mod json;
mod md;

use std::fs;
use std::path::PathBuf;

use common::DataExportConfig;
use tauri::State;
use time::macros::format_description;
use time::OffsetDateTime;

use crate::dataans::DataansState;
use crate::BACKUPS_DIR;

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn export_app_data(state: State<'_, DataansState>, export_config: DataExportConfig) -> Result<PathBuf, String> {
    let backups_dir = state.app_data_dir.join(BACKUPS_DIR);

    if !backups_dir.exists() {
        match fs::create_dir(&backups_dir) {
            Ok(()) => info!(?backups_dir, "Successfully created backups directory"),
            Err(err) => error!(?err, ?backups_dir, "Filed to create backups directory"),
        }
    }

    let format = format_description!("[year].[month].[day]-[hour].[minute].[second]");
    let backups_dir = backups_dir.join(
        OffsetDateTime::now_utc()
            .format(&format)
            .map_err(|err| format!("Cannot format datetime: {:?}", err))?,
    );

    fs::create_dir(&backups_dir)
        .map_err(|err| format!("Cannot create backups dir: {:?}. dir: {:?}", err, backups_dir))?;

    match export_config {
        DataExportConfig::Md(notes_export_option) => md::export(&notes_export_option, &backups_dir, &state.db)?,
        DataExportConfig::Json(schema_version) => json::export(schema_version, &backups_dir, &state.db)?,
    }

    Ok(backups_dir)
}
