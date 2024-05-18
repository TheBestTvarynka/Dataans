use common::space::Space;
use common::TOTES_PLUGIN_NAME;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::backend::{invoke, EmptyArgs};

pub async fn list_spaces() -> Result<Vec<Space<'static>>, String> {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let spaces = invoke(&format!("plugin:{}|list_spaces", TOTES_PLUGIN_NAME), args).await;
    info!("{:?}", spaces);

    Ok(from_value(spaces).expect("Spaces list deserialization from JsValue should not fail."))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSpaceArgs<'name> {
    space_data: Space<'name>,
}

pub async fn create_space(space_data: Space<'_>) -> Result<(), String> {
    let args = to_value(&CreateSpaceArgs { space_data }).expect("Space serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|create_space", TOTES_PLUGIN_NAME), args).await;
    info!("{:?}", result);

    Ok(())
}
