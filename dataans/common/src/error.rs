use serde::Serialize;

/// General error info.
///
/// The frontend does not need to know details of the error. A simple [String] is enough.
/// If the user wants more info about the error, they can find it in the dataans log file.
#[derive(Debug, Serialize)]
pub struct Error {
    /// Error description,
    pub dataans_error: String,
}
