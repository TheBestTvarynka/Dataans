use std::path::Path;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use uuid::Uuid;

use super::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileData<'name, 'data> {
    id: Uuid,
    name: &'name str,
    data: &'data [u8],
}

pub async fn upload_file(id: Uuid, name: &str, data: &[u8]) -> String {
    let args = to_value(&FileData { id, name, data }).expect("FileData serialization to JsValue should not fail.");
    let file_path = invoke("upload_file", args).await;

    let file_path: String = from_value(file_path).expect("Path object deserialization from JsValue should not fail.");
    file_path
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FilePath<'path> {
    path: &'path Path,
}

pub async fn remove_file(path: &Path) {
    let args = to_value(&FilePath { path }).expect("FilePath serialization to JsValue should not fail.");
    let _ = invoke("remove_file", args).await;
}

pub async fn open(path: &Path) {
    let args = to_value(&FilePath { path }).expect("FilePath serialization to JsValue should not fail.");
    let _ = invoke("open", args).await;
}

pub async fn reveal(path: &Path) {
    let args = to_value(&FilePath { path }).expect("FilePath serialization to JsValue should not fail.");
    let _ = invoke("reveal", args).await;
}
