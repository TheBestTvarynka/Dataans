// https://github.com/tauri-apps/plugins-workspace/blob/715a0477be8f6f77af0377f4eca2b649554446be/plugins/autostart/api-iife.js

use common::error::CommandResult;

use crate::backend::{EmptyArgs, invoke_command};

pub async fn enable() -> CommandResult<bool> {
    let _: () = invoke_command("plugin:autostart|enable", &EmptyArgs {}).await?;

    is_enabled().await
}

pub async fn disable() -> CommandResult<bool> {
    let _: () = invoke_command("plugin:autostart|disable", &EmptyArgs {}).await?;

    is_enabled().await
}

pub async fn is_enabled() -> CommandResult<bool> {
    let is_enabled = invoke_command("plugin:autostart|is_enabled", &EmptyArgs {}).await?;
    trace!("Is autostart enabled: {is_enabled:?}.");

    Ok(is_enabled)
}
