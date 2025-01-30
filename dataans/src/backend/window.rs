use common::error::CommandResultEmpty;
use serde::Serialize;
use url::Url;

use crate::backend::{invoke_command, EmptyArgs};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShowAppWindowArgs {
    pub web_server_url: Url,
}

pub async fn show_auth_window(web_server_url: Url) -> CommandResultEmpty {
    invoke_command("open_auth_window", &ShowAppWindowArgs { web_server_url }).await
}

pub async fn show_app_info_window() -> CommandResultEmpty {
    invoke_command("open_app_info_window", &EmptyArgs {}).await
}
