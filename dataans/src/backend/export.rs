use std::path::PathBuf;

use common::error::CommandResult;
use common::{DataExportConfig, APP_PLUGIN_NAME};
use serde::Serialize;

use crate::backend::invoke_command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportConfig {
    export_config: DataExportConfig,
}

pub async fn export_data(export_config: DataExportConfig) -> CommandResult<PathBuf> {
    invoke_command(
        &format!("plugin:{}|export_app_data", APP_PLUGIN_NAME),
        &ExportConfig { export_config },
    )
    .await
}
