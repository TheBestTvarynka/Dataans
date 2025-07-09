use common::error::CommandResult;
use common::profile::UserContext;
use common::APP_PLUGIN_NAME;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn profile() -> CommandResult<Option<UserContext>> {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|profile"), &EmptyArgs {}).await
}
