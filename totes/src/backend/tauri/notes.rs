use common::note::{Id as NoteId, Note, NoteFullOwned, UpdateNote};
use common::space::Id as SpaceId;
use common::TOTES_PLUGIN_NAME;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use super::invoke;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListNotesArgs {
    pub space_id: SpaceId,
}

pub async fn list_notes(space_id: SpaceId) -> Result<Vec<Note<'static>>, String> {
    let args = to_value(&ListNotesArgs { space_id }).expect("ListNotesArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|list_notes", TOTES_PLUGIN_NAME), args).await;

    Ok(from_value(notes).expect("Notes list deserialization from JsValue should not fail."))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteArgs<'text> {
    pub note: Note<'text>,
}

pub async fn create_note(note: Note<'_>) -> Result<(), String> {
    let args = to_value(&CreateNoteArgs { note }).expect("CreateNoteArgs serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|create_note", TOTES_PLUGIN_NAME), args).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteArgs<'text> {
    pub note_data: UpdateNote<'text>,
}

pub async fn update_note(note_data: UpdateNote<'_>) -> Result<(), String> {
    let args =
        to_value(&UpdateNoteArgs { note_data }).expect("UpdateNoteArgs serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|update_note", TOTES_PLUGIN_NAME), args).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteNoteArgs {
    pub note_id: NoteId,
}

pub async fn delete_note(note_id: NoteId) -> Result<(), String> {
    let args = to_value(&DeleteNoteArgs { note_id }).expect("DeleteNoteArgs serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|delete_note", TOTES_PLUGIN_NAME), args).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesInSpaceArgs<'query> {
    pub space_id: SpaceId,
    pub query: &'query str,
}

pub async fn search_notes_in_space(space_id: SpaceId, query: &str) -> Result<Vec<NoteFullOwned>, String> {
    let args = to_value(&SearchNotesInSpaceArgs { space_id, query })
        .expect("SearchNotesInSpaceArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|search_notes_in_space", TOTES_PLUGIN_NAME), args).await;

    Ok(from_value(notes).expect("Notes list deserialization from JsValue should not fail."))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesArgs<'query> {
    pub query: &'query str,
}

pub async fn search_notes(query: &str) -> Result<Vec<NoteFullOwned>, String> {
    let args = to_value(&SearchNotesArgs { query }).expect("SearchNotesArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|search_notes", TOTES_PLUGIN_NAME), args).await;

    Ok(from_value(notes).expect("Notes list deserialization from JsValue should not fail."))
}
