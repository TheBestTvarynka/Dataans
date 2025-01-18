use std::fmt;

use serde::{Deserialize, Serialize};

/// Error object returned from the Tauri command.
///
/// [CommandError] is shared between app frontend and backend.
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    /// Any error inside app backend.
    Dataans(String),
    /// Error during deserialization from [JsValue] or serialization into [JsValue].
    JsValue(String),
    /// Tauri error.
    Tauri(String),
    /// Invalid data,
    InvalidData(String),
    /// Event error,
    TauriEvent(String),
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        Self::Dataans(err.to_string())
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Dataans(m) => write!(f, "Dataans: {}", m),
            CommandError::JsValue(m) => write!(f, "JS value: {}", m),
            CommandError::Tauri(m) => write!(f, "Tauri: {}", m),
            CommandError::InvalidData(m) => write!(f, "invalid data: {}", m),
            CommandError::TauriEvent(_) => write!(f, "Tauri event error"),
        }
    }
}

/// Result type of the Tauri command.
pub type CommandResult<T> = Result<T, CommandError>;

/// Empty Tauri command result.
///
/// Use this type when the Tauri command should not return any data but may fail.
pub type CommandResultEmpty = Result<(), CommandError>;
