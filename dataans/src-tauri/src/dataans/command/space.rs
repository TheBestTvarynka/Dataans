use common::space::{DeleteSpace, OwnedSpace, UpdateSpace};
use tauri::State;

use super::CommandResult;
use crate::dataans::DataansState;

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> CommandResult<Vec<OwnedSpace>> {
    Ok(state.space_service.spaces().await.into())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn create_space(state: State<'_, DataansState>, space_data: OwnedSpace) -> CommandResult<()> {
    Ok(state.space_service.create_space(space_data).await.into())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn update_space(state: State<'_, DataansState>, space_data: UpdateSpace<'static>) -> CommandResult<()> {
    Ok(state.space_service.update_space(space_data).await.into())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> CommandResult<()> {
    Ok(state.space_service.delete_space(space_data).await.into())
}
