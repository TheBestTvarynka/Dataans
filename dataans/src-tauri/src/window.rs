use common::error::{CommandError, CommandResultEmpty};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const AUTH_WINDOW_TITLE: &str = "Auth-and-Sync";

#[instrument(level = "trace", ret, skip(app))]
#[tauri::command]
pub async fn open_auth_window(app: AppHandle) -> CommandResultEmpty {
    if let Some(window) = app.webview_windows().get(AUTH_WINDOW_TITLE) {
        warn!("Auth window already opened. Destroying...");

        window.destroy().map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    WebviewWindowBuilder::new(&app, AUTH_WINDOW_TITLE, WebviewUrl::App("auth".into()))
        .always_on_top(false)
        .decorations(true)
        .closable(true)
        .focused(true)
        .inner_size(800.0, 800.0)
        .title("Account")
        .build()
        .map_err(|err| CommandError::Tauri(err.to_string()))?;

    Ok(())
}
