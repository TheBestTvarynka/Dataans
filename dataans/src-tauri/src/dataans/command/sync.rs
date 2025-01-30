use common::error::{CommandResult, CommandResultEmpty};
use common::profile::{Sync, UserContext};
use tauri::{AppHandle, Emitter, Runtime, State};

use crate::dataans::command::web::emit_user_context;
use crate::dataans::{DataansError, DataansState};

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

#[tauri::command]
pub fn set_sync_options<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, DataansState>,
    sync_config: Sync,
) -> CommandResult<UserContext> {
    let user_context = state.web_service.set_sync_options(sync_config)?;

    emit_user_context(app, user_context.clone())?;

    Ok(user_context)
}
