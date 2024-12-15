use common::space::{DeleteSpace, OwnedSpace, UpdateSpace};
use tauri::State;

use crate::dataans::DataansState;

pub fn query_spaces() -> Vec<OwnedSpace> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> Result<Vec<OwnedSpace>, String> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn create_space(state: State<'_, DataansState>, space_data: OwnedSpace) -> Result<(), String> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn update_space(state: State<'_, DataansState>, space_data: UpdateSpace<'static>) -> Result<(), String> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> Result<(), String> {
    todo!()
}
