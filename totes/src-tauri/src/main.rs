// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust (Tauri)!", name)
}

use tauri::{
    AppHandle, CustomMenuItem, Manager, Result, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

const MAIN_WINDOW_NAME: &str = "main";

const WINDOW_VISIBILITY_MENU_ITEM_ID: &str = "visibility";
const WINDOW_QUIT_MENU_ITEM_ID: &str = "quit";
const WINDOW_HIDE_TITLE: &str = "Hide";
const WINDOW_SHOW_TITLE: &str = "Show";
const WINDOW_QUIT_TITLE: &str = "Quit";

fn toggle_app_visibility(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_window(MAIN_WINDOW_NAME) {
        let item_handle = app.tray_handle().get_item(WINDOW_VISIBILITY_MENU_ITEM_ID);

        if window.is_visible().unwrap_or(true) {
            window.hide()?;
            item_handle.set_title(WINDOW_SHOW_TITLE)?;
        } else {
            window.show()?;
            item_handle.set_title(WINDOW_HIDE_TITLE)?;
        }
    } else {
        println!("{MAIN_WINDOW_NAME} window not found!");
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
            SystemTrayEvent::LeftClick { .. }
            | SystemTrayEvent::DoubleClick { .. } => toggle_app_visibility(app).unwrap(),
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
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
