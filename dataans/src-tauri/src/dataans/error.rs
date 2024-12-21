use std::io::Error as IoError;
use std::path::PathBuf;

use serde::Serialize;
use thiserror::Error;

use crate::dataans::db::DbError;

#[derive(Debug, Error, Serialize)]
pub enum DataansError {
    #[serde(serialize_with = "serialize_db_error")]
    #[error("DbError: {0:?}")]
    DbError(#[from] DbError),

    #[serde(serialize_with = "serialize_io_error")]
    #[error("IoError: {0:?}")]
    IoError(#[from] IoError),

    #[serde(serialize_with = "serialize_json_error")]
    #[error("JsonError: {0:?}")]
    JsonError(#[from] serde_json::Error),

    #[serde(serialize_with = "serialize_time_format_error")]
    #[error("TimeFormatError: {0:?}")]
    TimeFormatError(#[from] time::error::Format),

    #[error("File or dir path is not UTF-8: {0:?}")]
    PathIsNotUtf8(PathBuf),

    #[serde(serialize_with = "serialize_clipboard_error")]
    #[error("Clipboard related error: {0:?}")]
    Clipboard(#[from] arboard::Error),

    #[serde(serialize_with = "serialize_image_error")]
    #[error("ImageError: {0:?}")]
    Image(#[from] image::ImageError),

    #[error("ImageGenerationError: {0}")]
    ImageGeneration(String),

    #[error("Can not create an image from raw image data")]
    ImageFromRaw,
}

macro_rules! serialize_err_as_str {
    ($err:ty, $fn_name:ident) => {
        pub fn $fn_name<S>(err: &$err, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            s.serialize_str(&format!("{:?}", err))
        }
    };
}

serialize_err_as_str!(DbError, serialize_db_error);
serialize_err_as_str!(IoError, serialize_io_error);
serialize_err_as_str!(serde_json::Error, serialize_json_error);
serialize_err_as_str!(time::error::Format, serialize_time_format_error);
serialize_err_as_str!(arboard::Error, serialize_clipboard_error);
serialize_err_as_str!(image::ImageError, serialize_image_error);
