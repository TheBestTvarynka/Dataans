use common::note::{Id as NoteId, Note};
use common::space::Id as SpaceId;
use common::TOTES_PLUGIN_NAME;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::backend::invoke;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListNotesArgs {
    pub space_id: SpaceId,
}

pub async fn list_notes(space_id: SpaceId) -> Result<Vec<Note<'static>>, String> {
    let args = to_value(&ListNotesArgs { space_id }).expect("ListNotesArgs serialization to JsValue should not fail.");
    let notes = invoke(&format!("plugin:{}|list_notes", TOTES_PLUGIN_NAME), args).await;
    info!("{:?}", notes);

    Ok(from_value(notes).expect("Notes list deserialization from JsValue should not fail."))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteArgs<'text> {
    pub note: Note<'text>,
}

pub async fn create_note(note: Note<'_>) -> Result<(), String> {
    let args = to_value(&CreateNoteArgs { note }).expect("CreateNoteArgs serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|create_note", TOTES_PLUGIN_NAME), args).await;
    info!("{:?}", result);

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteNoteArgs {
    pub note_id: NoteId,
}

pub async fn delete_note(note_id: NoteId) -> Result<(), String> {
    let args = to_value(&DeleteNoteArgs { note_id }).expect("DeleteNoteArgs serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|delete_note", TOTES_PLUGIN_NAME), args).await;
    info!("{:?}", result);

    Ok(())
}
