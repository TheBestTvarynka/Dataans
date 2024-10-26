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

pub fn convert_file_url(image_path: impl AsRef<str>) -> String {
    #[cfg(windows_is_host_os)]
    {
        image_path.as_ref()[24..].to_owned()
    }
    #[cfg(not(windows_is_host_os))]
    {
        image_path.as_ref()[18..].to_owned()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThemeFilepath<'path> {
    file_path: &'path Path,
}

pub async fn load_theme(file_path: &Path) -> Theme {
    let args = to_value(&ThemeFilepath { file_path }).expect("ThemeFilepath serialization to JsValue should not fail.");
    let theme_value = invoke("theme", args).await;

    from_value(theme_value).expect("Theme object deserialization from JsValue should not fail.")
}

#[derive(Serialize)]
struct EmptyArgs {}

pub async fn open_config_file() {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let _ = invoke("open_config_file", args).await;
}

pub async fn open_theme_file(file_path: &Path) {
    let args = to_value(&ThemeFilepath { file_path }).expect("ThemeFilepath serialization to JsValue should not fail.");
    let _ = invoke("open_theme_file", args).await;
}

pub async fn open_config_file_folder() {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let _ = invoke("open_config_file_folder", args).await;
}

pub async fn load_config() -> Config {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let theme_value = invoke("config", args).await;

    from_value(theme_value).expect("Config object deserialization from JsValue should not fail.")
}

pub async fn gen_avatar() -> String {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image_path = invoke("gen_random_avatar", args).await;

    let image_path: String =
        from_value(image_path).expect("PathBuf object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}

pub async fn load_clipboard_image() -> String {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let image_path = invoke("handle_clipboard_image", args).await;

    let image_path: String =
        from_value(image_path).expect("PathBuf object deserialization from JsValue should not fail.");
    convert_file_src(image_path)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParseCodeArgs<'lang, 'code> {
    lang: &'lang str,
    code: &'code str,
}

pub async fn parse_code(lang: &str, code: &str) -> String {
    let args =
        to_value(&ParseCodeArgs { lang, code }).expect("ParseCodeArgs serialization to JsValue should not fail.");
    let html = invoke("parse_code", args).await;

    from_value(html).expect("String object deserialization from JsValue should not fail.")
}
