use std::sync::Arc;

use common::error::{CommandResult, CommandResultEmpty};
use common::profile::{Sync, UserContext, UserProfile};
use tauri::{async_runtime, AppHandle, Emitter, Runtime, State};
use time::OffsetDateTime;

use crate::dataans::command::web::emit_user_context;
use crate::dataans::crypto::EncryptionKey;
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
pub async fn full_sync<R: Runtime>(app: AppHandle<R>, state: State<'_, DataansState>) -> CommandResultEmpty {
    let user_profile = if let Some(user_profile) = state.web_service.user_profile() {
        if user_profile.auth_token_expiration_date > OffsetDateTime::now_utc() {
            user_profile
        } else {
            return Err(DataansError::AuthTokenExpired.into());
        }
    } else {
        return Err(DataansError::UserNotSignedIn.into());
    };

    let operation_logger = Arc::clone(&state.operation_logger);

    let UserProfile {
        user_id: _,
        username: _,
        auth_token,
        auth_token_expiration_date: _,
        secret_key,
        sync_config,
    } = user_profile;

    async_runtime::spawn(async move {
        let _result = sync_future(
            operation_logger,
            sync_config.get_web_server_url().into(),
            auth_token,
            *EncryptionKey::from_slice(secret_key.as_ref().as_slice()),
            app,
        )
        .await;
    });

    Ok(())
}
