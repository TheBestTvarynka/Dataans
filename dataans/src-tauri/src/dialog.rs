use futures::channel::oneshot;
use std::path::PathBuf;
use tauri::{command, AppHandle};
use tauri_plugin_dialog::{DialogExt, FilePath};

#[command]
pub async fn open_file_dialog(app: AppHandle) -> Result<Option<PathBuf>, String> {
    // Create a oneshot channel to receive the result
    let (tx, rx) = oneshot::channel();

    // Spawn a task to open the dialog
    tauri::async_runtime::spawn(async move {
        app.dialog()
            .file()
            .add_filter("Notes", &["json", "md"])
            .pick_file(move |file_path| {
                let path = file_path.and_then(|fp| match fp {
                    FilePath::Path(p) => Some(p),
                    _ => None,
                });
                let _ = tx.send(path);
            });
    });

    // Wait for the result from the dialog
    rx.await.map_err(|e| format!("Failed to get file path: {}", e))
}
