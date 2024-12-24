use common::space::{DeleteSpace, OwnedSpace, Space, UpdateSpace};
use common::APP_PLUGIN_NAME;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use super::{from_js_value, DummyResult};
use crate::backend::{invoke, EmptyArgs};

pub async fn list_spaces() -> Result<Vec<OwnedSpace>, String> {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let spaces = invoke(&format!("plugin:{}|list_spaces", APP_PLUGIN_NAME), args).await;

    from_js_value(spaces)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSpaceArgs<'name, 'avatar> {
    space_data: Space<'name, 'avatar>,
}

pub async fn create_space(space_data: Space<'_, '_>) -> DummyResult {
    let args = to_value(&CreateSpaceArgs { space_data }).expect("Space serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|create_space", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceArgs<'name> {
    space_data: UpdateSpace<'name>,
}

pub async fn update_space(space_data: UpdateSpace<'_>) -> DummyResult {
    let args =
        to_value(&UpdateSpaceArgs { space_data }).expect("UpdateSpace serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|update_space", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteSpaceArgs {
    space_data: DeleteSpace,
}

pub async fn delete_space(space_data: DeleteSpace) -> DummyResult {
    let args =
        to_value(&DeleteSpaceArgs { space_data }).expect("DeleteSpace serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|delete_space", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}
