use std::path::PathBuf;

use common::{DataExportConfig, APP_PLUGIN_NAME};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};

use crate::backend::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportConfig {
    options: DataExportConfig,
}

pub async fn export_data(options: DataExportConfig) -> PathBuf {
    let args = to_value(&ExportConfig { options }).expect("ExportConfig serialization to JsValue should not fail.");
    let backup_path = invoke(&format!("plugin:{}|export_app_data", APP_PLUGIN_NAME), args).await;

    from_value(backup_path).expect("String list deserialization from JsValue should not fail.")
}
