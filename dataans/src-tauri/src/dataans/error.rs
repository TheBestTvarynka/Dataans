use std::io::Error as IoError;

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
