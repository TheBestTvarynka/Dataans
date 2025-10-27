use std::path::Path;

use common::APP_PLUGIN_NAME;
use common::error::{CommandResult, CommandResultEmpty};
use common::note::File;
use serde::Serialize;
use uuid::Uuid;

use crate::backend::{EmptyArgs, invoke_command};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileData<'name, 'data> {
    id: Uuid,
    name: &'name str,
    data: &'data [u8],
}

pub async fn upload_file(id: Uuid, name: &str, data: &[u8]) -> CommandResult<File> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|upload_file"),
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
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|delete_file"), &FileId { id }).await
}

pub async fn gen_avatar() -> CommandResult<File> {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|gen_random_avatar"), &EmptyArgs {}).await
}

pub async fn load_clipboard_image() -> CommandResult<File> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|handle_clipboard_image"),
        &EmptyArgs {},
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SafeFileAsArgs {
    file: File,
}

pub async fn save_file_as(file: File) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|save_file_as"),
        &SafeFileAsArgs { file },
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
