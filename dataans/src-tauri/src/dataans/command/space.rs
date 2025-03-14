use common::error::{CommandResult, CommandResultEmpty};
use common::space::{CreateSpaceOwned, DeleteSpace, OwnedSpace, UpdateSpace};
use tauri::State;

use crate::dataans::DataansState;

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> CommandResult<Vec<OwnedSpace>> {
    Ok(state.space_service.spaces().await?)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn create_space(state: State<'_, DataansState>, space_data: CreateSpaceOwned) -> CommandResult<OwnedSpace> {
    Ok(state.space_service.create_space(space_data).await?)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn update_space(
    state: State<'_, DataansState>,
    space_data: UpdateSpace<'static>,
) -> CommandResult<OwnedSpace> {
    Ok(state.space_service.update_space(space_data).await?)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> CommandResultEmpty {
    Ok(state.space_service.delete_space(space_data).await?)
}
