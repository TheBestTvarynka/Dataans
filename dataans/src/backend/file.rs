use std::path::Path;

use common::note::File;
use common::APP_PLUGIN_NAME;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use uuid::Uuid;

use super::{from_js_value, invoke, DummyResult, EmptyArgs};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileData<'name, 'data> {
    id: Uuid,
    name: &'name str,
    data: &'data [u8],
}

pub async fn upload_file(id: Uuid, name: &str, data: &[u8]) -> Result<File, String> {
    let args = to_value(&FileData { id, name, data }).expect("FileData serialization to JsValue should not fail.");
    let file = invoke(&format!("plugin:{}|upload_file", APP_PLUGIN_NAME), args).await;

    from_js_value(file)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileId {
    id: Uuid,
}

pub async fn remove_file(id: Uuid) -> DummyResult {
    let args = to_value(&FileId { id }).expect("FileId serialization to JsValue should not fail.");
    let result = invoke(&format!("plugin:{}|delete_file", APP_PLUGIN_NAME), args).await;

    from_js_value(result)
}

pub async fn gen_avatar() -> Result<File, String> {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image = invoke(&format!("plugin:{}|gen_random_avatar", APP_PLUGIN_NAME), args).await;

    from_js_value(image)
}

pub async fn load_clipboard_image() -> Result<File, String> {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image = invoke(&format!("plugin:{}|handle_clipboard_image", APP_PLUGIN_NAME), args).await;

    from_js_value(image)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FilePath<'path> {
    path: &'path Path,
}

pub async fn open(path: &Path) {
    let args = to_value(&FilePath { path }).expect("FilePath serialization to JsValue should not fail.");
    let _ = invoke("open", args).await;
}

pub async fn reveal(path: &Path) {
    let args = to_value(&FilePath { path }).expect("FilePath serialization to JsValue should not fail.");
    let _ = invoke("reveal", args).await;
}
