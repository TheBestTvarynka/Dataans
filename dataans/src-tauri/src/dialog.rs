use std::path::PathBuf;

use common::error::{CommandError, CommandResult};
use futures::channel::oneshot;
use tauri::{command, AppHandle};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tracing::error;

#[command]
pub async fn select_file(app: AppHandle) -> CommandResult<Option<PathBuf>> {
    let (tx, rx) = oneshot::channel();

    tauri::async_runtime::spawn(async move {
        app.dialog()
            .file()
            .add_filter("Notes", &["json"])
            .pick_file(move |file_path| {
                let result = match file_path {
                    Some(FilePath::Path(p)) => Ok(Some(p)),
                    Some(_) => {
                        let err = CommandError::Dataans("Unsupported file type selected".to_string());
                        error!("Failed to select file: {:?}", err);
                        Err(err)
                    }
                    None => Ok(None),
                };
                let _ = tx.send(result);
            });
    });

    match rx.await {
        Ok(result) => result,
        Err(e) => {
            let err = CommandError::Dataans(format!("Failed to receive file path: {}", e));
            error!("Failed to select file: {:?}", err);
            Err(err)
        }
    }
}
