use std::fmt;

use serde::{Deserialize, Serialize};

/// Command Error.
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    /// Dataans inner error.
    Dataans(String),
    /// Error parsing error.
    FromJsValue(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// TODO.
pub type CommandResult<T> = Result<T, CommandError>;
/// TODO.
pub type CommandResultEmpty = Result<(), CommandError>;
