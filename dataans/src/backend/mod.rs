pub mod autostart;
pub mod export;
pub mod file;
pub mod notes;
pub mod spaces;

use std::path::Path;

use common::{Config, Theme};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub type DummyResult = Result<common::error::DummyUnit, String>;

/// Accepts image fs path and returns its Tauri asset url.
pub fn convert_file_src(image_path: impl AsRef<str>) -> String {
    let image_path = image_path.as_ref();

    if image_path == common::DEFAULT_SPACE_AVATAR_PATH {
        return image_path.to_owned();
    }

    #[cfg(windows_is_host_os)]
    {
        format!("http://asset.localhost/{}", image_path)
    }
    #[cfg(not(windows_is_host_os))]
    {
        format!("asset://localhost/{}", image_path)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn from_js_value<T: serde::de::DeserializeOwned + std::fmt::Debug>(value: JsValue) -> Result<T, String> {
    use common::error::DataansResult;
    use serde_wasm_bindgen::from_value;

    from_value::<DataansResult<T>>(value)
        .expect("DataansResult deserialization should not fail")
        .into()
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
pub struct EmptyArgs {}

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
