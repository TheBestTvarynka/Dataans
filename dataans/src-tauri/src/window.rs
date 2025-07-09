use common::error::{CommandError, CommandResultEmpty};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

const APP_INFO_WINDOW_TITLE: &str = "App-Info";
const CF_WINDOW_TITLE: &str = "CF-Auth";

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

#[instrument(level = "trace", ret, skip(app))]
#[tauri::command]
pub async fn cf_auth(app: AppHandle, url: Url) -> CommandResultEmpty {
    if let Some(window) = app.webview_windows().get(CF_WINDOW_TITLE) {
        info!("CF-Auth window already opened");

        window.show().map_err(|err| CommandError::Tauri(err.to_string()))?;
        window.set_focus().map_err(|err| CommandError::Tauri(err.to_string()))?;
    } else {
        WebviewWindowBuilder::new(
            &app,
            CF_WINDOW_TITLE,
            WebviewUrl::External(url.join("health/authorize.html").expect("Invalid URL for CF-Auth")),
        )
        .always_on_top(false)
        .decorations(true)
        .closable(true)
        .focused(true)
        .inner_size(800.0, 800.0)
        .title("CF-Auth")
        .build()
        .map_err(|err| CommandError::Tauri(err.to_string()))?;
    }

    Ok(())
}
