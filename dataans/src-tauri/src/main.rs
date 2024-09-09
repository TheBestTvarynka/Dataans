// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate tracing;

mod code_block;
mod config;
mod dataans;
mod file;
mod image;

use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, Result, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

const MAIN_WINDOW_NAME: &str = "main";

const WINDOW_VISIBILITY_MENU_ITEM_ID: &str = "visibility";
const WINDOW_QUIT_MENU_ITEM_ID: &str = "quit";
const WINDOW_HIDE_TITLE: &str = "Hide";
const WINDOW_SHOW_TITLE: &str = "Show";
const WINDOW_QUIT_TITLE: &str = "Quit";

const IMAGED_DIR: &str = "images";
const FILES_DIR: &str = "files";
const CONFIGS_DIR: &str = "configs";
const LOGS_DIR: &str = "logs";

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

fn init_tracing() {
    use std::fs::OpenOptions;
    use std::{fs, io};

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    // STDOUT layer
    let stdout_layer = tracing_subscriber::fmt::layer().pretty().with_writer(io::stdout);

    let registry = tracing_subscriber::registry().with(stdout_layer);

    // `dataans.log` layer
    let context = tauri::generate_context!();
    let app_data = tauri::api::path::app_data_dir(context.config()).expect("APP_DATA directory should be defined.");

    if !app_data.exists() {
        match fs::create_dir(&app_data) {
            Ok(()) => println!("Successfully created app data directory: {:?}", app_data),
            Err(err) => eprintln!("Filed to create app data directory: {:?}. Path: {:?}", err, app_data),
        }
    }
    let logs_dir = app_data.join(LOGS_DIR);
    if !logs_dir.exists() {
        match fs::create_dir(&logs_dir) {
            Ok(()) => println!("Successfully created logs directory: {:?}", logs_dir),
            Err(err) => eprintln!("Filed to create logs directory: {:?}. Path: {:?}", err, logs_dir),
        }
    }

    let log_file = logs_dir.join("dataans.log");
    match OpenOptions::new().create(true).append(true).open(&log_file) {
        Ok(log_file) => {
            let log_file_layer = tracing_subscriber::fmt::layer().pretty().with_writer(log_file);

            registry.with(log_file_layer).with(EnvFilter::from_default_env()).init();
            return;
        }
        Err(e) => {
            eprintln!("Couldn't open log file: {e}. Path: {:?}.", log_file);
        }
    };

    registry.with(EnvFilter::from_default_env()).init();
}

fn main() {
    init_tracing();

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
        // .plugin(
        //     tauri_plugin_log::Builder::default()
        //         .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
        //         .build(),
        // )
        .plugin(dataans::init_dataans_plugin())
        .invoke_handler(tauri::generate_handler![
            config::theme,
            config::config,
            config::open,
            config::reveal,
            config::open_config_file,
            config::open_config_file_folder,
            config::open_theme_file,
            image::gen_random_avatar,
            image::handle_clipboard_image,
            file::upload_file,
            file::remove_file,
            code_block::parse_code
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                let app_handle = app_handle.clone();
                let app_toggle = crate::config::load_config_inner(&app_handle).app.app_toggle;
                debug!(?app_toggle);

                app_handle
                    .global_shortcut_manager()
                    .register(&app_toggle, move || {
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
