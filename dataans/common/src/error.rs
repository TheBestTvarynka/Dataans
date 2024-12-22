use std::fmt;

use serde::{Deserialize, Serialize};

/// Custom result type.
///
/// The frontend does not need to know details of the error. A simple [String] is enough.
/// If the user wants more info about the error, they can find it in the dataans log file.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataansResult<T> {
    ok: Option<T>,
    err: Option<String>,
}

impl<T> DataansResult<T> {
    /// Create ok.
    pub fn ok(ok: T) -> Self {
        Self {
            ok: Some(ok),
            err: None,
        }
    }

    /// Create err.
    pub fn err(err: String) -> Self {
        Self {
            ok: None,
            err: Some(err),
        }
    }

    /// Check result status.
    pub fn is_ok(&self) -> bool {
        self.ok.is_some()
    }
}

impl<T> From<DataansResult<T>> for Result<T, String> {
    fn from(result: DataansResult<T>) -> Result<T, String> {
        let DataansResult { ok, err } = result;

        if let Some(ok) = ok {
            Ok(ok)
        } else {
            Err(err.expect("Err obj should present"))
        }
    }
}

impl<T, E: fmt::Display> From<Result<T, E>> for DataansResult<T> {
    fn from(err: Result<T, E>) -> Self {
        match err {
            Ok(value) => DataansResult::ok(value),
            Err(err) => DataansResult::err(err.to_string()),
        }
    }
}
