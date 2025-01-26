use common::error::{CommandError, CommandResultEmpty};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

pub const AUTH_WINDOW_TITLE: &str = "Auth-and-Sync";
const APP_INFO_WINDOW_TITLE: &str = "App-Info";

#[instrument(level = "trace", ret, skip(app))]
#[tauri::command]
pub async fn open_auth_window(app: AppHandle, web_server_url: Url) -> CommandResultEmpty {
    if let Some(window) = app.webview_windows().get(AUTH_WINDOW_TITLE) {
        info!("Auth window already opened");

        window.show().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.set_focus().map_err(|err| CommandError::Tauri(err.to_string()))?;
    } else {
        let web_server_url = hex::encode(web_server_url.to_string());

        WebviewWindowBuilder::new(
            &app,
            AUTH_WINDOW_TITLE,
            WebviewUrl::App(format!("auth/{}", web_server_url).into()),
        )
        .always_on_top(false)
        .decorations(true)
        .closable(true)
        .focused(true)
        .inner_size(800.0, 800.0)
        .title("Account")
        .build()
        .map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    Ok(())
}

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
