use std::io::Error as IoError;
use std::path::PathBuf;

use common::error::CommandError;
use thiserror::Error;

use crate::dataans::db::DbError;
use crate::dataans::service::note::NoteServiceError;
use crate::dataans::service::space::SpaceServiceError;

#[derive(Debug, Error)]
pub enum DataansError {
    #[error("DB error: {0:?}")]
    DbError(#[from] DbError),

    #[error("IO error: {0:?}")]
    IoError(#[from] IoError),

    #[error("JSON error: {0:?}")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    NoteService(#[from] NoteServiceError),

    #[error(transparent)]
    SpaceService(#[from] SpaceServiceError),

    #[error("time format error: {0:?}")]
    TimeFormatError(#[from] time::error::Format),

    #[error("fs path is not UTF-8: {0:?}")]
    PathIsNotUtf8(PathBuf),

    #[error("clipboard error: {0:?}")]
    Clipboard(#[from] arboard::Error),

    #[error("image error: {0:?}")]
    Image(#[from] image::ImageError),

    #[error("image generation error: {0}")]
    ImageGeneration(String),

    #[error("can not create an image from raw image data")]
    ImageFromRaw,

    #[error("Incorrect import file type: only `json` is supported: {0}")]
    IncorrectImportFileType(String),

    #[error("failed to register the user: {0}")]
    SignUpFailed(String),

    #[error("failed to parse the url: {0:?}")]
    ParseUrl(#[from] url::ParseError),

    #[error("failed to send a request: {0:?}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to sign in: {0}")]
    SignInFailed(String),

    #[error("failed to read secret-key file: path {0}, error: {1:?}")]
    SecretKeyFile(PathBuf, IoError),

    #[error("failed to parse secret key: {0:?}")]
    ParseSecretKey(hex::FromHexError),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error("user is not signed in")]
    UserNotSignedIn,
}

impl From<DataansError> for CommandError {
    fn from(error: DataansError) -> Self {
        Self::Dataans(error.to_string())
    }
}
