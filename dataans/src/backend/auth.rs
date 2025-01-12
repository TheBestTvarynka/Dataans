use common::error::CommandResultEmpty;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn show_auth_window() -> CommandResultEmpty {
    invoke_command("open_auth_window", &EmptyArgs {}).await
}
