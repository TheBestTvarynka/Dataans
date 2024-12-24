use std::path::PathBuf;

use common::{DataExportConfig, APP_PLUGIN_NAME};
use serde::Serialize;
use serde_wasm_bindgen::to_value;

use super::from_js_value;
use crate::backend::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportConfig {
    export_config: DataExportConfig,
}

pub async fn export_data(export_config: DataExportConfig) -> Result<PathBuf, String> {
    let args =
        to_value(&ExportConfig { export_config }).expect("ExportConfig serialization to JsValue should not fail.");
    let backup_path = invoke(&format!("plugin:{}|export_app_data", APP_PLUGIN_NAME), args).await;

    from_js_value(backup_path)
}
