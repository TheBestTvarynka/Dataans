pub mod auth;
pub mod autostart;
pub mod export;
pub mod file;
pub mod notes;
pub mod spaces;
pub mod sync;
pub mod window;

use std::path::Path;

use common::error::{CommandError, CommandResult, CommandResultEmpty};
use common::{Config, Theme};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Accepts image fs path and returns its Tauri asset url.
pub fn convert_file_src(image_path: impl AsRef<str>, base_path: impl AsRef<str>) -> String {
    let image_path = image_path.as_ref();
    let base_path = base_path.as_ref();

    if image_path == common::DEFAULT_SPACE_AVATAR_PATH {
        return image_path.to_owned();
    }

    #[cfg(windows_is_host_os)]
    {
        format!("http://asset.localhost/{base_path}/files/{image_path}")
    }
    #[cfg(not(windows_is_host_os))]
    {
        format!("asset://localhost/{base_path}/files/{image_path}")
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn invoke_command<T: serde::de::DeserializeOwned>(command: &str, args: &impl Serialize) -> CommandResult<T> {
    let theme_value = invoke(
        command,
        to_value(&args).map_err(|err| CommandError::JsValue(err.to_string()))?,
    )
    .await
    .map_err(|err| from_value::<CommandError>(err).unwrap_or_else(|err| CommandError::JsValue(err.to_string())))?;

    from_value(theme_value).map_err(|err| CommandError::JsValue(err.to_string()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThemeFilepath<'path> {
    file_path: &'path Path,
}

pub async fn load_theme(file_path: &Path) -> CommandResult<Theme> {
    invoke_command("theme", &ThemeFilepath { file_path }).await
}

#[derive(Serialize)]
pub struct EmptyArgs {}

pub async fn open_config_file() {
    let _: CommandResultEmpty = invoke_command("open_config_file", &EmptyArgs {}).await;
}

pub async fn open_theme_file(file_path: &Path) {
    let _: CommandResultEmpty = invoke_command("open_theme_file", &ThemeFilepath { file_path }).await;
}

pub async fn open_config_file_folder() {
    let _: CommandResultEmpty = invoke_command("open_config_file_folder", &EmptyArgs {}).await;
}

pub async fn load_config() -> CommandResult<Config> {
    invoke_command("config", &EmptyArgs {}).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParseCodeArgs<'lang, 'code> {
    lang: &'lang str,
    code: &'code str,
}

pub async fn parse_code(lang: &str, code: &str) -> CommandResult<String> {
    invoke_command("parse_code", &ParseCodeArgs { lang, code }).await
}
