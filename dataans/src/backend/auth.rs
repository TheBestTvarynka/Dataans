use common::common_api_types::{InvitationToken, Password, Username};
use common::error::{CommandError, CommandResult, CommandResultEmpty};
use common::APP_PLUGIN_NAME;
use serde::Serialize;
use uuid::Uuid;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn show_auth_window() -> CommandResultEmpty {
    invoke_command("open_auth_window", &EmptyArgs {}).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignUpArgs {
    pub invitation_token: InvitationToken,
    pub username: Username,
    pub password: Password,
}

pub async fn sign_up(invitation_token: Vec<u8>, username: String, password: String) -> CommandResult<Uuid> {
    invoke_command(
        &format!("plugin:{}|sign_up", APP_PLUGIN_NAME),
        &SignUpArgs {
            invitation_token: invitation_token
                .try_into()
                .map_err(|_| CommandError::InvalidData("Invalid invitation token".into()))?,
            username: username
                .try_into()
                .map_err(|_| CommandError::InvalidData("Invalid username".into()))?,
            password: password
                .try_into()
                .map_err(|_| CommandError::InvalidData("Bad password".into()))?,
        },
    )
    .await
}
