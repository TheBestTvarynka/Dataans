// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate log;

mod config;
mod totes;

use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, Result, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_log::LogTarget;

const MAIN_WINDOW_NAME: &str = "main";

const WINDOW_VISIBILITY_MENU_ITEM_ID: &str = "visibility";
const WINDOW_QUIT_MENU_ITEM_ID: &str = "quit";
const WINDOW_HIDE_TITLE: &str = "Hide";
const WINDOW_SHOW_TITLE: &str = "Show";
const WINDOW_QUIT_TITLE: &str = "Quit";

// TODO: make it configurable.
const GLOBAL_SHORTCUT_ACCELERATOR: &str = "F1";

fn toggle_app_visibility(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_window(MAIN_WINDOW_NAME) {
        let item_handle = app.tray_handle().get_item(WINDOW_VISIBILITY_MENU_ITEM_ID);

        if window.is_visible().unwrap_or(true) {
            info!("Hide main window");
            window.hide()?;
            item_handle.set_title(WINDOW_SHOW_TITLE)?;
        } else {
            info!("Show main window");
            window.show()?;
            item_handle.set_title(WINDOW_HIDE_TITLE)?;
        }
    } else {
        error!("{MAIN_WINDOW_NAME} window not found!");
    }

    Ok(())
}

fn main() {
    let quit = CustomMenuItem::new(WINDOW_QUIT_MENU_ITEM_ID.to_string(), WINDOW_QUIT_TITLE);
    let hide = CustomMenuItem::new(WINDOW_VISIBILITY_MENU_ITEM_ID.to_string(), WINDOW_HIDE_TITLE);
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } | SystemTrayEvent::DoubleClick { .. } => {
                toggle_app_visibility(app).unwrap()
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                WINDOW_QUIT_MENU_ITEM_ID => {
                    std::process::exit(0);
                }
                WINDOW_VISIBILITY_MENU_ITEM_ID => toggle_app_visibility(app).unwrap(),
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .plugin(totes::init_totes_plugin())
        .invoke_handler(tauri::generate_handler![config::theme,])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                let app_handle = app_handle.clone();
                app_handle
                    .global_shortcut_manager()
                    .register(GLOBAL_SHORTCUT_ACCELERATOR, move || {
                        info!("global shortcut: {}", GLOBAL_SHORTCUT_ACCELERATOR);
                        toggle_app_visibility(&app_handle).unwrap();
                    })
                    .unwrap();
            }
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
