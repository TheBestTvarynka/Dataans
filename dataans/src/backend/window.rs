use common::error::CommandResultEmpty;
use serde::Serialize;
use url::Url;

use crate::backend::{EmptyArgs, invoke_command};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShowAuthWindowArgs<'a> {
    pub url: &'a Url,
}

pub async fn show_cf_auth_window(url: &Url) -> CommandResultEmpty {
    invoke_command("cf_auth", &ShowAuthWindowArgs { url }).await
}

pub async fn show_app_info_window() -> CommandResultEmpty {
    invoke_command("open_app_info_window", &EmptyArgs {}).await
}
