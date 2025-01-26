use common::error::{CommandResult, CommandResultEmpty, CommandError};
use common::profile::{SecretKey, WebServerUrl};
use tauri::{State, AppHandle, Runtime, Manager};
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, Username};

use crate::dataans::DataansState;

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
    state
        .web_service
        .sign_in(secret_key, username, password, web_server_url)
        .await?;

    if let Some(window) = app.webview_windows().get(crate::window::AUTH_WINDOW_TITLE) {
        info!("Auth window present. Closing it...");

        window.close().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.destroy().map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    Ok(())
}
