use common::error::Error as CommonError;
use common::note::{Id as NoteId, NoteFullOwned, OwnedNote, UpdateNote};
use common::space::Id as SpaceId;
use tauri::State;

use crate::dataans::DataansState;

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn list_notes(state: State<'_, DataansState>, space_id: SpaceId) -> Result<Vec<OwnedNote>, CommonError> {
    Ok(state.note_service.space_notes(space_id).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn create_note(state: State<'_, DataansState>, note: OwnedNote) -> Result<(), CommonError> {
    Ok(state.note_service.create_note(note).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn update_note(state: State<'_, DataansState>, note_data: UpdateNote<'_>) -> Result<(), CommonError> {
    Ok(state.note_service.update_note(note_data).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn delete_note(state: State<'_, DataansState>, note_id: NoteId) -> Result<(), CommonError> {
    Ok(state.note_service.delete_note(note_id).await?)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn search_notes_in_space(
    state: State<'_, DataansState>,
    query: String,
    space_id: SpaceId,
) -> Result<Vec<NoteFullOwned>, CommonError> {
    Ok(state.note_service.search_notes_in_space(&query, space_id).await?)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn search_notes(state: State<'_, DataansState>, query: String) -> Result<Vec<NoteFullOwned>, CommonError> {
    Ok(state.note_service.search_notes(&query).await?)
}
