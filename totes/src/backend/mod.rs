pub mod file;
pub mod notes;
pub mod spaces;

use std::path::Path;

use common::{Config, Theme};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

fn convert_file_src(image_path: impl AsRef<str>) -> String {
    #[cfg(windows_is_host_os)]
    {
        format!("https://asset.localhost/{}", image_path.as_ref())
    }
    #[cfg(not(windows_is_host_os))]
    {
        format!("asset://localhost/{}", image_path.as_ref())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThemeFilepath<'path> {
    file_path: &'path Path,
}

pub async fn load_theme(file_path: &Path) -> Theme {
    let args = to_value(&ThemeFilepath { file_path }).expect("EmptyArgs serialization to JsValue should not fail.");
    let theme_value = invoke("theme", args).await;

    from_value(theme_value).expect("Theme object deserialization from JsValue should not fail.")
}

#[derive(Serialize)]
struct EmptyArgs {}

pub async fn load_config() -> Config {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let theme_value = invoke("config", args).await;

    from_value(theme_value).expect("Config object deserialization from JsValue should not fail.")
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageData<'name, 'data> {
    image_name: &'name str,
    image_data: &'data [u8],
}

pub async fn save_image(image_name: &str, image_data: &[u8]) -> String {
    let args =
        to_value(&ImageData { image_name, image_data }).expect("ImageData serialization to JsValue should not fail.");
    let image_path = invoke("save_image", args).await;

    let image_path: String = from_value(image_path).expect("Path object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}

pub async fn gen_avatar() -> String {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image_path = invoke("gen_random_avatar", args).await;

    let image_path: String =
        from_value(image_path).expect("Theme object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}
