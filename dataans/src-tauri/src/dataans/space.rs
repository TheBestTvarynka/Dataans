use common::space::{DeleteSpace, OwnedSpace, UpdateSpace};
use polodb_core::bson::doc;
use tauri::State;

use crate::dataans::{DataansState, SPACES_COLLECTION_NAME};

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub async fn list_spaces(state: State<'_, DataansState>) -> Result<Vec<OwnedSpace>, String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let mut spaces = Vec::new();
    for space in collection.find(None).expect("Spaces querying should not fail.") {
        spaces.push(space.unwrap());
    }

    Ok(spaces)
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn create_space(state: State<'_, DataansState>, space_data: OwnedSpace) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    collection
        .insert_one(space_data)
        .expect("space insertion should not fail");

    Ok(())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn update_space(state: State<'_, DataansState>, space_data: UpdateSpace<'static>) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let _ = collection
        .update_one(
            doc! {
                "id": space_data.id.inner().to_string(),
            },
            doc! {
                "$set": doc! {
                    "name": space_data.name.as_ref(),
                    "avatar": space_data.avatar.as_ref(),
                }
            },
        )
        .expect("space insertion should not fail");

    Ok(())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn delete_space(state: State<'_, DataansState>, space_data: DeleteSpace) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let _ = collection
        .delete_one(doc! {
            "id": space_data.id.inner().to_string(),
        })
        .expect("space deletion should not fail");

    Ok(())
}
