mod event;

use common::error::CommandResultEmpty;
use common::event::{UserContextEvent, USER_CONTEXT_EVENT};
use common::profile::UserContext;
use common::APP_PLUGIN_NAME;
use futures::StreamExt;
use leptos::{Callable, Callback};

use crate::backend::{invoke_command, EmptyArgs};

pub async fn on_user_context(set_user_context: impl Fn(Option<UserContext>) -> ()) -> CommandResultEmpty {
    let mut events = event::listen::<UserContextEvent>(USER_CONTEXT_EVENT).await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);

        match event.payload {
            UserContextEvent::SignedIn(user_context) => {
                set_user_context(Some(user_context));
            }
            UserContextEvent::SignedOut => {
                set_user_context(None);
            }
        }
    }

    Ok(())
}

pub async fn trigger_sync() -> CommandResultEmpty {
    invoke_command(&format!("plugin:{}|sync", APP_PLUGIN_NAME), &EmptyArgs {}).await
}
