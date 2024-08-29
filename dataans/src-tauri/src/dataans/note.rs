use std::collections::HashMap;

use common::note::{Id as NoteId, Note, NoteFull, NoteFullOwned, UpdateNote};
use common::space::{Id as SpaceId, OwnedSpace};
use polodb_core::bson::doc;
use tauri::State;

use crate::dataans::{DataansState, NOTES_COLLECTION_NAME, SPACES_COLLECTION_NAME};

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn list_notes(state: State<'_, DataansState>, space_id: SpaceId) -> Result<Vec<Note>, String> {
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

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn create_note(state: State<'_, DataansState>, note: Note<'static>) -> Result<(), String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    collection.insert_one(note).expect("Note insertion should not fail");

    Ok(())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn update_note(state: State<'_, DataansState>, note_data: UpdateNote<'_>) -> Result<(), String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    let _ = collection
        .update_one(
            doc! {
                "id": note_data.id.inner().to_string(),
            },
            doc! {
                "$set": doc! {
                    "text": note_data.text.as_ref(),
                    "files": &note_data.files,
                }
            },
        )
        .expect("Note updating should not fail");

    Ok(())
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn delete_note(state: State<'_, DataansState>, note_id: NoteId) -> Result<(), String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

    let _ = collection
        .delete_one(doc! {
            "id": note_id.inner().to_string(),
        })
        .expect("note deletion should not fail");

    Ok(())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn search_notes_in_space(
    state: State<'_, DataansState>,
    query: String,
    space_id: SpaceId,
) -> Result<Vec<NoteFullOwned>, String> {
    let notes_collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);
    let spaces_collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let mut notes = Vec::new();
    for note in notes_collection
        .find(doc! {
            "space_id": space_id.inner().to_string(),
        })
        .expect("Space notes querying should not fail")
    {
        let note = note.expect("Note parsing should not fail.");
        if note.text.as_ref().contains(&query) {
            let space = if let Some(space) = spaces_collection
                .find_one(doc! {
                    "id": note.space_id.inner().to_string(),
                })
                .expect("Space notes querying should not fail")
            {
                space
            } else {
                warn!(
                    "Space(id={}) does not exist. Skipping this note(id={})...",
                    note.space_id.inner().to_string(),
                    note.id.to_string()
                );
                continue;
            };

            notes.push(NoteFull {
                id: note.id,
                text: note.text,
                created_at: note.created_at,
                space,
                files: note.files,
            });
        }
    }

    Ok(notes)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn search_notes(state: State<'_, DataansState>, query: String) -> Result<Vec<NoteFullOwned>, String> {
    let collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);
    let spaces_collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let mut spaces = HashMap::<SpaceId, OwnedSpace>::new();
    let mut notes = Vec::new();

    for note in collection.find(None).expect("Notes querying should not fail") {
        let note = note.expect("Note parsing should not fail.");
        if note.text.as_ref().contains(&query) {
            let space = if let Some(space) = spaces.get(&note.space_id) {
                space.clone()
            } else {
                let space = if let Some(space) = spaces_collection
                    .find_one(doc! {
                        "id": note.space_id.inner().to_string(),
                    })
                    .expect("Space notes querying should not fail")
                {
                    spaces.insert(note.space_id, space.clone());
                    space
                } else {
                    warn!(
                        "Space(id={}) does not exist. Skipping this note(id={})...",
                        note.space_id.inner().to_string(),
                        note.id.to_string()
                    );
                    continue;
                };
                space
            };

            notes.push(NoteFull {
                id: note.id,
                text: note.text,
                created_at: note.created_at,
                space,
                files: note.files,
            });
        }
    }

    Ok(notes)
}
