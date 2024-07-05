use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};

use super::{convert_file_src, invoke};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileData<'name, 'data> {
    name: &'name str,
    data: &'data [u8],
}

pub async fn upload_file(name: &str, data: &[u8]) -> String {
    let args = to_value(&FileData { name, data }).expect("FileData serialization to JsValue should not fail.");
    let file_path = invoke("upload_file", args).await;

    let file_path: String = from_value(file_path).expect("Path object deserialization from JsValue should not fail.");
    convert_file_src(file_path)
}
