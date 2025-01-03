use std::path::Path;

use common::error::{CommandResult, CommandResultEmpty};
use common::note::File;
use common::APP_PLUGIN_NAME;
use serde::Serialize;
use uuid::Uuid;

use crate::backend::{invoke_command, EmptyArgs};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileData<'name, 'data> {
    id: Uuid,
    name: &'name str,
    data: &'data [u8],
}

pub async fn upload_file(id: Uuid, name: &str, data: &[u8]) -> CommandResult<File> {
    invoke_command(
        &format!("plugin:{}|upload_file", APP_PLUGIN_NAME),
        &FileData { id, name, data },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileId {
    id: Uuid,
}

pub async fn remove_file(id: Uuid) -> CommandResultEmpty {
    invoke_command(&format!("plugin:{}|delete_file", APP_PLUGIN_NAME), &FileId { id }).await
}

pub async fn gen_avatar() -> CommandResult<File> {
    invoke_command(&format!("plugin:{}|gen_random_avatar", APP_PLUGIN_NAME), &EmptyArgs {}).await
}

pub async fn load_clipboard_image() -> CommandResult<File> {
    invoke_command(
        &format!("plugin:{}|handle_clipboard_image", APP_PLUGIN_NAME),
        &EmptyArgs {},
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FilePath<'path> {
    path: &'path Path,
}

pub async fn open(path: &Path) {
    let _: CommandResultEmpty = invoke_command("open", &FilePath { path }).await;
}

pub async fn reveal(path: &Path) {
    let _: CommandResultEmpty = invoke_command("reveal", &FilePath { path }).await;
}
