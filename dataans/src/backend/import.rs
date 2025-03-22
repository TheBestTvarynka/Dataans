use common::{error::CommandResult, APP_PLUGIN_NAME};
use serde::Serialize;

use crate::backend::invoke_command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportConfig {
    path: String,
}

pub async fn open_file_dialog() -> CommandResult<String> {
    invoke_command(&format!("open_file_dialog"), &()).await
}

pub async fn import_app_data(path: String) -> CommandResult<()> {
    invoke_command(
        &format!("plugin:{}|import_app_data", APP_PLUGIN_NAME),
        &ImportConfig { path },
    )
    .await
}
