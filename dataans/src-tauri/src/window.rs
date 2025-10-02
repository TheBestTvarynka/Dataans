use std::path::PathBuf;

use common::error::{CommandError, CommandResult, CommandResultEmpty};
use futures::channel::oneshot;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, FilePath};
use url::Url;

use crate::dataans::error::DataansError;

const APP_INFO_WINDOW_TITLE: &str = "App-Info";
pub const CF_WINDOW_TITLE: &str = "CF-Auth";

/// Opens the App-Info window.
///
/// This window contains basic app information and lists current settings.
#[instrument(level = "trace", ret, skip(app))]
#[tauri::command]
pub async fn open_app_info_window(app: AppHandle) -> CommandResultEmpty {
    if let Some(window) = app.webview_windows().get(APP_INFO_WINDOW_TITLE) {
        info!("App info window already opened");

        window.show().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.set_focus().map_err(|err| CommandError::Tauri(err.to_string()))?;
    } else {
        WebviewWindowBuilder::new(&app, APP_INFO_WINDOW_TITLE, WebviewUrl::App("app-info".into()))
            .always_on_top(false)
            .decorations(true)
            .closable(true)
            .focused(true)
            .inner_size(800.0, 800.0)
            .title("App info")
            .build()
            .map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    Ok(())
}

/// Opens the CF-Auth window.
///
/// It is used for authenticating to Cloudflare Zero Trust Access.
#[instrument(level = "trace", ret, skip(app))]
#[tauri::command]
pub async fn cf_auth<R: Runtime>(app: AppHandle<R>, url: Url) -> CommandResultEmpty {
    if let Some(window) = app.webview_windows().get(CF_WINDOW_TITLE) {
        info!("CF-Auth window already opened");

        window.clear_all_browsing_data().map_err(DataansError::from)?;

        window.show().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.set_focus().map_err(|err| CommandError::Tauri(err.to_string()))?;
    } else {
        let window = WebviewWindowBuilder::new(
            &app,
            CF_WINDOW_TITLE,
            WebviewUrl::External(url.join("health/authorize.html").expect("Invalid URL for CF-Auth")),
        )
        .always_on_top(false)
        .decorations(true)
        .closable(true)
        .focused(true)
        .inner_size(800.0, 800.0)
        .title(CF_WINDOW_TITLE)
        .build()
        .map_err(|err| CommandError::Tauri(err.to_string()))?;

        window.clear_all_browsing_data().map_err(DataansError::from)?;
    }

    Ok(())
}

/// Selects the data file for importing into the app.
///
/// Currently, only the json import is supported.
#[tauri::command]
pub async fn select_import_file(app: AppHandle) -> CommandResult<Option<PathBuf>> {
    let (tx, rx) = oneshot::channel();

    tauri::async_runtime::spawn(async move {
        app.dialog()
            .file()
            .add_filter("Notes", &["json"])
            .pick_file(move |file_path| {
                let result = match file_path {
                    Some(FilePath::Path(p)) => Ok(Some(p)),
                    Some(_) => {
                        let err = CommandError::Dataans("unsupported file type selected".to_string());
                        error!(?err, "Failed to select file");
                        Err(err)
                    }
                    None => Ok(None),
                };
                let _ = tx.send(result);
            });
    });

    match rx.await {
        Ok(result) => result,
        Err(e) => {
            let err = CommandError::Dataans(format!("failed to receive file path: {e}"));
            error!(?err, "failed to select file");
            Err(err)
        }
    }
}
