use common::error::CommandResult;
use common::APP_PLUGIN_NAME;
use serde::Serialize;

use super::EmptyArgs;
use crate::backend::invoke_command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportConfig {
    path: String,
}

pub async fn select_import_file() -> CommandResult<Option<String>> {
    invoke_command("select_import_file", &EmptyArgs {}).await
}

pub async fn import_app_data(path: String) -> CommandResult<()> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|import_app_data"),
        &ImportConfig { path },
    )
    .await
}
