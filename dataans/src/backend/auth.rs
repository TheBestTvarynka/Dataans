use common::error::{CommandResult, CommandResultEmpty};
use common::profile::UserContext;
use common::APP_PLUGIN_NAME;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn profile() -> CommandResult<Option<UserContext>> {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|profile"), &EmptyArgs {}).await
}

pub async fn sign_out() -> CommandResultEmpty {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|sign_out"), &EmptyArgs {}).await
}
