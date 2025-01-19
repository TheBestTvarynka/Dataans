use common::error::CommandResultEmpty;
use tauri::{AppHandle, Emitter, Runtime};

use crate::dataans::DataansError;

async fn sync_inner<R: Runtime>(app: AppHandle<R>) -> Result<(), DataansError> {
    app.emit_to("main", "sync", String::from("tbt"))?;

    for progress in [1, 15, 50, 80, 100] {
        app.emit_to("main", "sync", format!("{}", progress))?;
    }
    app.emit_to("main", "sync", String::from("thebesttvarynka"))?;

    Ok(())
}

#[tauri::command]
pub async fn sync<R: Runtime>(app: AppHandle<R>) -> CommandResultEmpty {
    sync_inner(app).await?;

    Ok(())
}
