use std::sync::Arc;

use common::error::{CommandResult, CommandResultEmpty};
use common::event::{StatusUpdateEvent, STATUS_UPDATE_EVENT};
use common::profile::{Sync, UserContext, UserProfile};
use tauri::{async_runtime, AppHandle, Emitter, Runtime, State};
use time::OffsetDateTime;

use crate::dataans::command::web::emit_user_context;
use crate::dataans::crypto::EncryptionKey;
use crate::dataans::sync::sync_future;
use crate::dataans::{DataansError, DataansState};

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
        let status_update_event = sync_future(
            operation_logger,
            sync_config.get_web_server_url().into(),
            auth_token,
            *EncryptionKey::from_slice(secret_key.as_ref().as_slice()),
            &app,
        )
        .await
        .map(|_| StatusUpdateEvent::SyncSuccessful)
        .unwrap_or_else(|err| StatusUpdateEvent::SyncFailed(err.to_string()));

        if let Err(err) = app.emit(STATUS_UPDATE_EVENT, status_update_event) {
            error!(?err, "Failed to emit status update event");
        };
    });

    Ok(())
}
