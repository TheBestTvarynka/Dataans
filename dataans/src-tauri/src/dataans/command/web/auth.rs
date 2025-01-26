use common::error::{CommandResult, CommandResultEmpty};
use common::profile::{SecretKey, WebServerUrl};
use tauri::State;
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
pub async fn sign_in(
    state: State<'_, DataansState>,
    secret_key: Option<SecretKey>,
    username: Username,
    password: Password,
    web_server_url: WebServerUrl,
) -> CommandResultEmpty {
    Ok(state
        .web_service
        .sign_in(secret_key, username, password, web_server_url)
        .await?)
}
