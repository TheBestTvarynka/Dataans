mod event;

use common::error::{CommandResult, CommandResultEmpty};
use common::event::{
    DataEvent, StatusUpdateEvent, UserContextEvent, DATA_EVENT, STATUS_UPDATE_EVENT, USER_CONTEXT_EVENT,
};
use common::profile::{Sync, UserContext};
use common::APP_PLUGIN_NAME;
use futures::StreamExt;
use leptoaster::ToasterContext;
use leptos::{RwSignal, SignalUpdate};
use serde::Serialize;

use crate::backend::{invoke_command, EmptyArgs};
use crate::GlobalState;

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

pub async fn on_status_update(toaster: ToasterContext) -> CommandResultEmpty {
    let mut events = event::listen::<StatusUpdateEvent>(STATUS_UPDATE_EVENT).await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);

        match event.payload {
            StatusUpdateEvent::SyncSuccessful => {
                toaster.toast(
                    leptoaster::ToastBuilder::new("Synchronization successful.")
                        .with_level(leptoaster::ToastLevel::Success)
                        .with_position(leptoaster::ToastPosition::BottomRight)
                        .with_expiry(Some(3000)),
                );
            }
            StatusUpdateEvent::SyncFailed(message) => {
                error!("{:?}", message);
                toaster.toast(
                    leptoaster::ToastBuilder::new(&format!("Synchronization failed: {message}"))
                        .with_level(leptoaster::ToastLevel::Error)
                        .with_position(leptoaster::ToastPosition::BottomRight)
                        .with_expiry(Some(5000)),
                );
            }
        }
    }

    Ok(())
}

pub async fn on_data(data: RwSignal<GlobalState>) -> CommandResultEmpty {
    let mut events = event::listen::<DataEvent>(DATA_EVENT).await?;

    while let Some(event) = events.next().await {
        info!("Event received: {:?}", event);

        match event.payload {
            DataEvent::FileStatusUpdated(file_id, file_status) => {
                data.update(|state| {
                    state.notes.iter_mut().for_each(|note| {
                        if let Some(file) = note.files.iter_mut().find(|f| f.id == file_id) {
                            file.status = file_status;
                        }
                    });
                });
            }
            DataEvent::FileAdded(file) => {
                debug!("File added: {:?}", file);
                // Nothing to do here.
            }
            DataEvent::SpaceAdded(space) => {
                data.update(|state| {
                    state.spaces.push(space);
                });
            }
            DataEvent::SpaceUpdated(space) => {
                data.update(|state| {
                    if let Some(local_space) = state.spaces.iter_mut().find(|s| s.id == space.id) {
                        *local_space = space;
                    } else {
                        warn!("Received space update event for space that does not exist: {:?}", space);
                    }
                });
            }
            DataEvent::SpaceDeleted(space_id) => {
                data.update(|state| {
                    state.spaces.retain(|s| s.id != space_id);
                });
            }
            DataEvent::NoteAdded(note) => {
                data.update(|state| {
                    if state
                        .selected_space
                        .as_ref()
                        .map(|selected_space| selected_space.id == note.space_id)
                        .unwrap_or(false)
                    {
                        state.notes.push(note);
                        state.notes.sort_by(|a, b| a.created_at.cmp(&b.created_at));
                    } else {
                        trace!(
                            "Received update note event for a space that is not selected: {:?}",
                            note
                        );
                    }
                });
            }
            DataEvent::NoteUpdated(note) => {
                data.update(|state| {
                    if let Some(local_note) = state.notes.iter_mut().find(|n| n.id == note.id) {
                        *local_note = note;
                    }
                });
            }
            DataEvent::NoteDeleted(_space_id, note_id) => {
                data.update(|state| {
                    state.notes.retain(|n| n.id != note_id);
                });
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

#[allow(dead_code)]
pub async fn set_sync_options(sync_config: &Sync) -> CommandResult<UserContext> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|set_sync_options"),
        &SyncConfigArgs { sync_config },
    )
    .await
}

pub async fn trigger_full_sync() -> CommandResultEmpty {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|full_sync"), &EmptyArgs {}).await
}
