use common::space::{DeleteSpace, OwnedSpace, Space, UpdateSpace};
use common::APP_PLUGIN_NAME;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::backend::{invoke, EmptyArgs};

pub async fn list_spaces() -> Result<Vec<OwnedSpace>, String> {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let spaces = invoke(&format!("plugin:{}|list_spaces", APP_PLUGIN_NAME), args).await;

    Ok(from_value(spaces).expect("Spaces list deserialization from JsValue should not fail."))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSpaceArgs<'name, 'avatar> {
    space_data: Space<'name, 'avatar>,
}

pub async fn create_space(space_data: Space<'_, '_>) -> Result<(), String> {
    debug!("Creating space: {:?}", space_data);
    let args = to_value(&CreateSpaceArgs { space_data }).expect("Space serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|create_space", APP_PLUGIN_NAME), args).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceArgs<'name> {
    space_data: UpdateSpace<'name>,
}

pub async fn update_space(space_data: UpdateSpace<'_>) -> Result<(), String> {
    let args =
        to_value(&UpdateSpaceArgs { space_data }).expect("UpdateSpace serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|update_space", APP_PLUGIN_NAME), args).await;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteSpaceArgs {
    space_data: DeleteSpace,
}

pub async fn delete_space(space_data: DeleteSpace) -> Result<(), String> {
    let args =
        to_value(&DeleteSpaceArgs { space_data }).expect("DeleteSpace serialization to JsValue should not fail.");
    let _ = invoke(&format!("plugin:{}|delete_space", APP_PLUGIN_NAME), args).await;

    Ok(())
}
