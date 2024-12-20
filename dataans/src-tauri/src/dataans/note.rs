use std::collections::HashMap;

use common::note::{Id as NoteId, Note, NoteFull, NoteFullOwned, UpdateNote};
use common::space::{Id as SpaceId, OwnedSpace};
use tauri::State;

use crate::dataans::{DataansState, Db};

pub fn query_space_notes(space_id: SpaceId) -> Vec<Note<'static>> {
    todo!()
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn list_notes(state: State<'_, DataansState>, space_id: SpaceId) -> Result<Vec<Note>, String> {
    todo!()
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn create_note(state: State<'_, DataansState>, note: Note<'static>) -> Result<(), String> {
    todo!()
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn update_note(state: State<'_, DataansState>, note_data: UpdateNote<'_>) -> Result<(), String> {
    todo!()
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub fn delete_note(state: State<'_, DataansState>, note_id: NoteId) -> Result<(), String> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn search_notes_in_space(
    state: State<'_, DataansState>,
    query: String,
    space_id: SpaceId,
) -> Result<Vec<NoteFullOwned>, String> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn search_notes(state: State<'_, DataansState>, query: String) -> Result<Vec<NoteFullOwned>, String> {
    todo!()
}
