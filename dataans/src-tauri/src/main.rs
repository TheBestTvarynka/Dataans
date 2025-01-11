// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate tracing;

mod code_block;
mod config;
mod dataans;

use std::path::Path;
use std::str::FromStr;

use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::{AppHandle, Manager, Result, RunEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

const LOGGING_ENV_VAR_NAME: &str = "DATAANS_LOG";
const DEFAULT_LOG_LEVEL: &str = "dataans=trace";

const MAIN_WINDOW_NAME: &str = "main";

const WINDOW_VISIBILITY_MENU_ITEM_ID: &str = "visibility";
const WINDOW_QUIT_MENU_ITEM_ID: &str = "quit";
const WINDOW_VISIBILITY_TITLE: &str = "Toggle";
const WINDOW_QUIT_TITLE: &str = "Quit";

const PROFILE_DIR: &str = "profile";
const IMAGES_DIR: &str = "images";
const FILES_DIR: &str = "files";
const CONFIGS_DIR: &str = "configs";
const CONFIG_FILE_NAME: &str = "config.toml";
const LOGS_DIR: &str = "logs";
const BACKUPS_DIR: &str = "backups";

fn toggle_app_visibility(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_NAME) {
        if window.is_visible().unwrap_or(false) {
            info!("Hide main window");
            window.hide()?;
        } else {
            info!("Show main window");
            window.show()?;
            window.set_focus()?;
        }
    } else {
        error!("{MAIN_WINDOW_NAME} window not found!");
    }

    Ok(())
}

fn init_tracing(app_data: &Path) {
    use std::fs::OpenOptions;
    use std::{fs, io};

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let logging_filter: EnvFilter = EnvFilter::builder()
        .with_default_directive(DEFAULT_LOG_LEVEL.parse().expect("Default log level constant is bad."))
        .with_env_var(LOGGING_ENV_VAR_NAME)
        .from_env_lossy();

    // STDOUT layer
    let stdout_layer = tracing_subscriber::fmt::layer().pretty().with_writer(io::stdout);

    let registry = tracing_subscriber::registry().with(stdout_layer);

    // `dataans.log` layer
    if !app_data.exists() {
        match fs::create_dir(app_data) {
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
            registry.with(log_file_layer).with(logging_filter).init();
        }
        Err(e) => {
            eprintln!("Couldn't open log file: {e}. Path: {:?}.", log_file);
            registry.with(logging_filter).init();
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(dataans::init_dataans_plugin())
        .setup(|app| {
            init_tracing(&app.path().app_data_dir()?);

            // Set up system tray
            let toggle_visibility =
                MenuItemBuilder::with_id(WINDOW_VISIBILITY_MENU_ITEM_ID, WINDOW_VISIBILITY_TITLE).build(app)?;
            let quit = MenuItemBuilder::with_id(WINDOW_QUIT_MENU_ITEM_ID, WINDOW_QUIT_TITLE).build(app)?;

            let menu = MenuBuilder::new(app).items(&[&toggle_visibility, &quit]).build()?;

            if let Some(tray_icon) = app.tray_by_id("main") {
                tray_icon.set_menu(Some(menu)).unwrap();
                tray_icon.on_menu_event(move |app, event| match event.id().as_ref() {
                    WINDOW_VISIBILITY_MENU_ITEM_ID => toggle_app_visibility(app).unwrap(),
                    WINDOW_QUIT_MENU_ITEM_ID => {
                        info!("Exiting the app...");

                        app.cleanup_before_exit();
                        std::process::exit(0);
                    }
                    event_id => warn!(?event_id, "Unknown tray event id"),
                });
            } else {
                warn!("Cannot find the 'main' try icon :(");
            }

            let config = crate::config::load_config_inner(app.handle()).expect("config reading should not fail");

            // Set up global shortcut
            let visibility_shortcut = Shortcut::from_str(&config.app.app_toggle).unwrap();
            debug!(?visibility_shortcut);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        info!("Handle global shortcut.");
                        if *shortcut == visibility_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    debug!("Global visibility shortcut has been pressed.");
                                    toggle_app_visibility(app).unwrap();
                                }
                                ShortcutState::Released => {
                                    debug!("Global visibility shortcut has been released.");
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(visibility_shortcut)?;

            if config.app.always_on_top {
                if let Some(window) = app.handle().get_webview_window(MAIN_WINDOW_NAME) {
                    window.set_always_on_top(true)?;
                } else {
                    error!("{MAIN_WINDOW_NAME} window not found! Cannot set 'always-on-top'.");
                }
            }

            if config.app.hide_window_decorations {
                if let Some(window) = app.handle().get_webview_window(MAIN_WINDOW_NAME) {
                    window.set_decorations(false)?;
                } else {
                    error!("{MAIN_WINDOW_NAME} window not found! Cannot set 'hide-window-decorations'.");
                }
            }

            if config.app.hide_taskbar_icon {
                if let Some(window) = app.handle().get_webview_window(MAIN_WINDOW_NAME) {
                    window.set_skip_taskbar(true)?;
                } else {
                    error!("{MAIN_WINDOW_NAME} window not found! Cannot set 'hide-taskbar-icon'.");
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            config::theme,
            config::config,
            config::open,
            config::reveal,
            config::open_config_file,
            config::open_config_file_folder,
            config::open_theme_file,
            code_block::parse_code,
        ])
        .build(
            {
                #![allow(deprecated)]
                tauri::generate_context!()
            },
        )
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        })
}
