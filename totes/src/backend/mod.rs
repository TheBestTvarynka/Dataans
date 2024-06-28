pub mod notes;
pub mod spaces;

use common::Theme;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

fn convert_file_src(image_path: impl AsRef<str>) -> String {
    format!("https://asset.localhost/{}", image_path.as_ref())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct EmptyArgs {}

pub async fn load_theme() -> Theme {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let theme_value = invoke("theme", args).await;

    from_value(theme_value).expect("Theme object deserialization from JsValue should not fail.")
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageName {
    image_name: String,
}

pub async fn image_path(image_name: String) -> String {
    let args = to_value(&ImageName { image_name }).expect("ImageName serialization to JsValue should not fail.");
    let image_path = invoke("image_path", args).await;

    let image_path: String =
        from_value(image_path).expect("Theme object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}

pub async fn gen_avatar() -> String {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image_path = invoke("gen_random_avatar", args).await;

    let image_path: String =
        from_value(image_path).expect("Theme object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}
