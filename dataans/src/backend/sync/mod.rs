mod event;

use common::error::{CommandResult, CommandResultEmpty};
use common::event::{UserContextEvent, USER_CONTEXT_EVENT};
use common::profile::{Sync, UserContext};
use common::APP_PLUGIN_NAME;
use futures::StreamExt;
use serde::Serialize;

use crate::backend::invoke_command;

pub async fn on_user_context(set_user_context: impl Fn(Option<UserContext>)) -> CommandResultEmpty {
    let mut events = event::listen::<UserContextEvent>(USER_CONTEXT_EVENT).await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);

        match event.payload {
            UserContextEvent::SignedIn(user_context) => {
                set_user_context(Some(user_context));
            }
            UserContextEvent::ContextUpdated(user_context) => {
                set_user_context(Some(user_context));
            }
            UserContextEvent::SignedOut => {
                set_user_context(None);
            }
        }
    }

    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncConfigArgs<'a> {
    pub sync_config: &'a Sync,
}

pub async fn set_sync_options(sync_config: &Sync) -> CommandResult<UserContext> {
    invoke_command(
        &format!("plugin:{}|set_sync_options", APP_PLUGIN_NAME),
        &SyncConfigArgs { sync_config },
    )
    .await
}
