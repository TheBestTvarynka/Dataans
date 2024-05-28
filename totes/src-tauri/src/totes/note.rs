use common::note::{Id as NoteId, Note};
use common::space::Id as SpaceId;
use polodb_core::bson::doc;
use tauri::State;

use crate::totes::{TotesState, NOTES_COLLECTION_NAME};

#[tauri::command]
pub fn list_notes(state: State<'_, TotesState>, space_id: SpaceId) -> Result<Vec<Note>, String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    let mut notes = Vec::new();
    for note in collection
        .find(doc! {
            "space_id": space_id.inner().to_string(),
        })
        .expect("Space notes querying should not fail")
    {
        notes.push(note.expect("Note parsing should not fail."));
    }

    Ok(notes)
}

#[tauri::command]
pub fn create_note(state: State<'_, TotesState>, note: Note<'static>) -> Result<(), String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    collection.insert_one(note).expect("Note insertion should not fail");

    Ok(())
}

#[tauri::command]
pub fn delete_note(state: State<'_, TotesState>, note_id: NoteId) -> Result<(), String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    let _ = collection
        .delete_one(doc! {
            "id": note_id.inner().to_string(),
        })
        .expect("note deletion should not fail");

    Ok(())
}
