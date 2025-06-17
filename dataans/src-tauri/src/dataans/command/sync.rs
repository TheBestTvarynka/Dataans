use std::sync::Arc;

use common::error::{CommandResult, CommandResultEmpty};
use common::profile::{Sync, UserContext, UserProfile};
use tauri::{async_runtime, AppHandle, Emitter, Runtime, State};
use url::Url;

use crate::dataans::command::web::emit_user_context;
use crate::dataans::sync::sync_future;
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

#[tauri::command]
pub async fn full_sync<R: Runtime>(_app: AppHandle<R>, state: State<'_, DataansState>) -> CommandResultEmpty {
    let user_profile = if let Some(user_profile) = state.web_service.user_profile() {
        user_profile
    } else {
        return Err(DataansError::UserNotSignedIn.into());
    };

    let operation_logger = Arc::clone(&state.operation_logger);

    let UserProfile {
        user_id: _,
        username: _,
        auth_token,
        auth_token_expiration_date: _,
        secret_key: _,
        sync_config: _,
    } = user_profile;

    // TODO: derive encryption key properly.
    let encryption_key = [
        19, 28, 59, 181, 9, 202, 41, 22, 25, 122, 144, 217, 9, 87, 170, 209, 72, 223, 145, 41, 12, 252, 9, 229, 45,
        218, 206, 161, 199, 216, 243, 53,
    ];

    async_runtime::spawn(async move {
        let _result = sync_future(
            operation_logger,
            Url::parse("http://127.0.0.1:8000/").unwrap(),
            auth_token,
            encryption_key.into(),
        )
        .await;
    });

    Ok(())
}
