use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    /// DbError.
    DbError(String),
    /// IO error.
    IoError(String),
    /// JSON error.
    JsonError(String),
    /// Time format error.
    TimeFormatError(String),
    /// Path is not UTF-8.
    PathIsNotUtf8(PathBuf),
    /// Clipboard error.
    Clipboard(String),
    /// Image error.
    Image(String),
    /// Image generation error.
    ImageGeneration(String),
    /// Image from raw error.
    ImageFromRaw,
    /// Other.
    Other(String),
}

/// TODO.
pub type CommandResult<T> = Result<T, CommandError>;
/// TODO.
pub type CommandResultEmpty = Result<(), CommandError>;
