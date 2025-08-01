use std::sync::Arc;

use common::error::{CommandResult, CommandResultEmpty};
use common::event::{STATUS_UPDATE_EVENT, StatusUpdateEvent};
use common::profile::{Sync, UserContext, UserProfile};
use tauri::{AppHandle, Emitter, Runtime, State, async_runtime};

use crate::dataans::command::auth::emit_user_context;
use crate::dataans::crypto::EncryptionKey;
use crate::dataans::sync::sync_future;
use crate::dataans::{DataansError, DataansState};

#[tauri::command]
pub async fn set_sync_options<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, DataansState>,
    sync_config: Sync,
) -> CommandResult<UserContext> {
    let user_context = state.web_service.set_sync_options(sync_config).await?;

    emit_user_context(&app, user_context.clone())?;

    Ok(user_context)
}

#[tauri::command]
pub async fn full_sync<R: Runtime>(app: AppHandle<R>, state: State<'_, DataansState>) -> CommandResultEmpty {
    let Some(user_profile) = state.web_service.user_profile() else {
        return Err(DataansError::UserNotSignedIn.into());
    };

    let operation_logger = Arc::clone(&state.operation_logger);
    let files_path = Arc::clone(&state.files_path);

    let UserProfile {
        auth_token,
        secret_key,
        sync_config,
        salt: _,
    } = user_profile;

    async_runtime::spawn(async move {
        let status_update_event = sync_future(
            operation_logger,
            sync_config.url.into(),
            *EncryptionKey::from_slice(secret_key.as_ref().as_slice()),
            &app,
            files_path,
            &auth_token,
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
