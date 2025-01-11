use common::error::{CommandResult, CommandResultEmpty};
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
) -> CommandResult<Uuid> {
    Ok(state.web_service.sign_up(invitation_token, username, password).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn sign_in(state: State<'_, DataansState>, username: Username, password: Password) -> CommandResultEmpty {
    Ok(state.web_service.sign_in(username, password).await?)
}
