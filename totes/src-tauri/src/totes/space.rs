use common::space::{DeleteSpace, OwnedSpace, Space, UpdateSpace};
use polodb_core::bson::doc;
use tauri::State;

use crate::totes::{TotesState, SPACES_COLLECTION_NAME};

#[tauri::command]
pub async fn list_spaces(state: State<'_, TotesState>) -> Result<Vec<OwnedSpace>, String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let mut spaces = Vec::new();
    for space in collection.find(None).expect("Spaces querying should not fail.") {
        spaces.push(space.unwrap());
    }

    Ok(spaces)
}

#[tauri::command]
pub fn create_space(state: State<'_, TotesState>, space_data: OwnedSpace) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    collection
        .insert_one(space_data)
        .expect("space insertion should not fail");

    Ok(())
}

#[tauri::command]
pub fn update_space(state: State<'_, TotesState>, space_data: UpdateSpace<'static>) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let _ = collection
        .update_one(
            doc! {
                "id": space_data.id.inner().to_string(),
            },
            doc! {
                "$set": doc! {
                    "name": space_data.name.as_ref(),
                }
            },
        )
        .expect("space insertion should not fail");

    Ok(())
}

#[tauri::command]
pub fn delete_space(state: State<'_, TotesState>, space_data: DeleteSpace) -> Result<(), String> {
    let collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);

    let _ = collection
        .delete_one(doc! {
            "id": space_data.id.inner().to_string(),
        })
        .expect("space deletion should not fail");

    Ok(())
}
