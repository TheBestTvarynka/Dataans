use std::io::Error as IoError;
use std::path::PathBuf;

use common::error::CommandError;
use thiserror::Error;

use crate::dataans::db::DbError;

#[derive(Debug, Error)]
pub enum DataansError {
    #[error("DbError: {0:?}")]
    DbError(#[from] DbError),

    #[error("IoError: {0:?}")]
    IoError(#[from] IoError),

    #[error("JsonError: {0:?}")]
    JsonError(#[from] serde_json::Error),

    #[error("TimeFormatError: {0:?}")]
    TimeFormatError(#[from] time::error::Format),

    #[error("File or dir path is not UTF-8: {0:?}")]
    PathIsNotUtf8(PathBuf),

    #[error("Clipboard related error: {0:?}")]
    Clipboard(#[from] arboard::Error),

    #[error("ImageError: {0:?}")]
    Image(#[from] image::ImageError),

    #[error("ImageGenerationError: {0}")]
    ImageGeneration(String),

    #[error("Can not create an image from raw image data")]
    ImageFromRaw,
}

impl From<DataansError> for CommandError {
    fn from(error: DataansError) -> Self {
        Self::Dataans(error.to_string())
    }
}
