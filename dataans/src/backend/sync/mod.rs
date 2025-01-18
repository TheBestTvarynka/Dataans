mod event;

use common::error::CommandResultEmpty;
use futures::StreamExt;

pub async fn simple_listen() -> CommandResultEmpty {
    let mut events = event::listen::<String>("sync").await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);
    }

    Ok(())
}
