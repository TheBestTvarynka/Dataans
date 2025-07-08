use common::error::CommandResultEmpty;
use serde::Serialize;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn show_cf_auth_window() -> CommandResultEmpty {
    invoke_command("cf_auth", &EmptyArgs {}).await
}

pub async fn show_app_info_window() -> CommandResultEmpty {
    invoke_command("open_app_info_window", &EmptyArgs {}).await
}
