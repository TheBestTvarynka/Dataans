use common::error::{CommandError, CommandResult, CommandResultEmpty};
use common::event::{UserContextEvent, USER_CONTEXT_EVENT};
use common::profile::{SecretKey, UserContext, WebServerUrl};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, Username};

use crate::dataans::{DataansError, DataansState};

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn sign_up(
    state: State<'_, DataansState>,
    invitation_token: InvitationToken,
    username: Username,
    password: Password,
    web_server_url: WebServerUrl,
) -> CommandResult<Uuid> {
    Ok(state
        .web_service
        .sign_up(invitation_token, username, password, web_server_url)
        .await?)
}

fn emit_user_context<R: Runtime>(app: AppHandle<R>, user_context: UserContext) -> Result<(), DataansError> {
    app.emit(USER_CONTEXT_EVENT, UserContextEvent::SignedIn(user_context))?;

    Ok(())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn sign_in<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, DataansState>,
    secret_key: Option<SecretKey>,
    username: Username,
    password: Password,
    web_server_url: WebServerUrl,
) -> CommandResultEmpty {
    let user_context = state
        .web_service
        .sign_in(secret_key, username, password, web_server_url)
        .await?;

    if let Some(window) = app.webview_windows().get(crate::window::AUTH_WINDOW_TITLE) {
        info!("Auth window present. Closing it...");

        window.close().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.destroy().map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    emit_user_context(app, user_context)?;

    Ok(())
}
