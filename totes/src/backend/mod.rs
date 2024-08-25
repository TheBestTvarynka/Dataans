#[cfg(all(feature = "browser_backend", feature = "tauri_backend"))]
compile_error!("Only one backend feature can be enabled at a time.");

#[cfg(feature = "browser_backend")]
mod browser;
#[cfg(feature = "tauri_backend")]
mod tauri;

#[cfg(feature = "browser_backend")]
pub use browser::*;
#[cfg(feature = "tauri_backend")]
pub use tauri::*;
