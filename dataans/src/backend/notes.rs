use common::error::{CommandResult, CommandResultEmpty};
use common::note::{Id as NoteId, Note, NoteFullOwned, OwnedNote, UpdateNote};
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
        &format!("plugin:{}|list_notes", APP_PLUGIN_NAME),
        &ListNotesArgs { space_id },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteArgs<'text> {
    pub note: Note<'text>,
}

pub async fn create_note(note: Note<'_>) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{}|create_note", APP_PLUGIN_NAME),
        &CreateNoteArgs { note },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteArgs<'text> {
    pub note_data: UpdateNote<'text>,
}

pub async fn update_note(note_data: UpdateNote<'_>) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{}|update_note", APP_PLUGIN_NAME),
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
        &format!("plugin:{}|delete_note", APP_PLUGIN_NAME),
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
        &format!("plugin:{}|search_notes_in_space", APP_PLUGIN_NAME),
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
        &format!("plugin:{}|search_notes", APP_PLUGIN_NAME),
        &SearchNotesArgs { query },
    )
    .await
}
