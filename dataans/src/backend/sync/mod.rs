mod event;

use common::error::CommandResultEmpty;
use common::APP_PLUGIN_NAME;
use futures::StreamExt;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn simple_listen() -> CommandResultEmpty {
    let mut events = event::listen::<String>("sync").await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);
    }

    Ok(())
}

pub async fn trigger_sync() -> CommandResultEmpty {
    invoke_command(&format!("plugin:{}|sync", APP_PLUGIN_NAME), &EmptyArgs {}).await
}
