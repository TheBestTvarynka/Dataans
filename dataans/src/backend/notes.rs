use common::note::{Id as NoteId, Note, NoteFullOwned, OwnedNote, UpdateNote};
use common::space::Id as SpaceId;
use common::APP_PLUGIN_NAME;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

use super::from_js_value;
use crate::backend::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListNotesArgs {
    pub space_id: SpaceId,
}

pub async fn list_notes(space_id: SpaceId) -> Result<Vec<OwnedNote>, String> {
    let args = to_value(&ListNotesArgs { space_id }).expect("ListNotesArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|list_notes", APP_PLUGIN_NAME), args).await;

    from_js_value(notes)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteArgs<'text> {
    pub note: Note<'text>,
}

pub async fn create_note(note: Note<'_>) -> Result<(), String> {
    let args = to_value(&CreateNoteArgs { note }).expect("CreateNoteArgs serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|create_note", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteArgs<'text> {
    pub note_data: UpdateNote<'text>,
}

pub async fn update_note(note_data: UpdateNote<'_>) -> Result<(), String> {
    let args =
        to_value(&UpdateNoteArgs { note_data }).expect("UpdateNoteArgs serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|update_note", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteNoteArgs {
    pub note_id: NoteId,
}

pub async fn delete_note(note_id: NoteId) -> Result<(), String> {
    let args = to_value(&DeleteNoteArgs { note_id }).expect("DeleteNoteArgs serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|delete_note", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesInSpaceArgs<'query> {
    pub space_id: SpaceId,
    pub query: &'query str,
}

pub async fn search_notes_in_space(space_id: SpaceId, query: &str) -> Result<Vec<NoteFullOwned>, String> {
    let args = to_value(&SearchNotesInSpaceArgs { space_id, query })
        .expect("SearchNotesInSpaceArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|search_notes_in_space", APP_PLUGIN_NAME), args).await;

    from_js_value(notes)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchNotesArgs<'query> {
    pub query: &'query str,
}

pub async fn search_notes(query: &str) -> Result<Vec<NoteFullOwned>, String> {
    let args = to_value(&SearchNotesArgs { query }).expect("SearchNotesArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|search_notes", APP_PLUGIN_NAME), args).await;

    from_js_value(notes)
}
