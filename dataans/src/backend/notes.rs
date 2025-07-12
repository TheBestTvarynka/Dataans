use common::error::{CommandResult, CommandResultEmpty};
use common::note::{CreateNote, Id as NoteId, NoteFullOwned, OwnedNote, UpdateNote};
use common::space::Id as SpaceId;
use common::APP_PLUGIN_NAME;
use serde::Serialize;

use crate::backend::invoke_command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListNotesArgs {
    pub space_id: SpaceId,
}

pub async fn list_notes(space_id: SpaceId) -> CommandResult<Vec<OwnedNote>> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|list_notes"),
        &ListNotesArgs { space_id },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteArgs<'text> {
    pub note: CreateNote<'text>,
}

pub async fn create_note(note: CreateNote<'_>) -> CommandResult<OwnedNote> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|create_note"),
        &CreateNoteArgs { note },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteArgs<'text> {
    pub note_data: UpdateNote<'text>,
}

pub async fn update_note(note_data: UpdateNote<'_>) -> CommandResult<OwnedNote> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|update_note"),
        &UpdateNoteArgs { note_data },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteNoteArgs {
    pub note_id: NoteId,
}

pub async fn delete_note(note_id: NoteId) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|delete_note"),
        &DeleteNoteArgs { note_id },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesInSpaceArgs<'query> {
    pub space_id: SpaceId,
    pub query: &'query str,
}

pub async fn search_notes_in_space(space_id: SpaceId, query: &str) -> CommandResult<Vec<NoteFullOwned>> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|search_notes_in_space"),
        &SearchNotesInSpaceArgs { space_id, query },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesArgs<'query> {
    pub query: &'query str,
}

pub async fn search_notes(query: &str) -> CommandResult<Vec<NoteFullOwned>> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|search_notes"),
        &SearchNotesArgs { query },
    )
    .await
}
