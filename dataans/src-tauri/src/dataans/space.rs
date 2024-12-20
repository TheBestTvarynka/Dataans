use common::space::{DeleteSpace, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;
use tauri::State;

use crate::dataans::db::model::Space as SpaceModel;
use crate::dataans::db::Db;
use crate::dataans::{DataansError, DataansState};

pub async fn query_spaces<T: Db>(db: &T) -> Result<Vec<OwnedSpace>, DataansError> {
    async fn map_model_space_to_space<T: Db>(space: SpaceModel, db: &T) -> Result<OwnedSpace, DataansError> {
        let SpaceModel {
            id,
            name,
            avatar_id,
            created_at,
        } = space;

        let avatar = db.file_by_id(avatar_id).await?;

        Ok(OwnedSpace {
            id: id.into(),
            name: name.into(),
            avatar: avatar.path.into(),
            created_at: created_at.into(),
        })
    }

    let spaces = try_join_all(
        db.spaces()
            .await?
            .into_iter()
            .map(|space| map_model_space_to_space(space, db)),
    )
    .await?;

    Ok(spaces)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> Result<Vec<OwnedSpace>, DataansError> {
    query_spaces(&state.db).await
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn create_space(state: State<'_, DataansState>, space_data: OwnedSpace) -> Result<(), DataansError> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn update_space(state: State<'_, DataansState>, space_data: UpdateSpace<'static>) -> Result<(), DataansError> {
    todo!()
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> Result<(), DataansError> {
    todo!()
}
