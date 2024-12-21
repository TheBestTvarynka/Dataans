use common::space::{DeleteSpace, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;
use tauri::State;

use crate::dataans::db::model::Space as SpaceModel;
use crate::dataans::db::Db;
use crate::dataans::{DataansError, DataansState};

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> Result<Vec<OwnedSpace>, DataansError> {
    state.space_service.spaces().await
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn create_space(state: State<'_, DataansState>, space_data: OwnedSpace) -> Result<(), DataansError> {
    state.space_service.create_space(space_data).await
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn update_space(
    state: State<'_, DataansState>,
    space_data: UpdateSpace<'static>,
) -> Result<(), DataansError> {
    state.space_service.update_space(space_data).await
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> Result<(), DataansError> {
    state.space_service.delete_space(space_data).await
}