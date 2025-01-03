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
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Result type of the Tauri command.
pub type CommandResult<T> = Result<T, CommandError>;

/// Empty Tauri command result.
///
/// Use this type when the Tauri command should not return any data but may fail.
pub type CommandResultEmpty = Result<(), CommandError>;
