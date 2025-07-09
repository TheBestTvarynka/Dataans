use thiserror::Error;

use crate::db::DbError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DbError: {0:?}")]
    DbError(DbError),

    #[error("the requested resource not found")]
    NotFound,

    #[error("invalid {0}")]
    InvalidData(&'static str),

    #[error("internal error: {0}")]
    Internal(&'static str),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("file saver error: {0}")]
    FileSaver(String),

    #[error("unauthorized: {0}")]
    Unauthorized(&'static str),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("json web token error: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
}

impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        if let DbError::SqlxError(sqlx::Error::RowNotFound) = err {
            Self::NotFound
        } else {
            Self::DbError(err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for web_api_types::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::DbError(err) => {
                error!(?err);
                Self::DbError("interaction with the database failed".into())
            }
            Error::NotFound => Self::NotFound("the requested resource not found".into()),
            Error::Internal(err) => {
                error!(err);
                Self::Internal("internal error".into())
            }
            Error::InvalidData(_) => Self::InvalidData(error.to_string()),
            Error::Io(_) => Self::Internal("internal IO error".into()),
            Error::FileSaver(_) => Self::Internal("internal file saver error".into()),
            Error::Unauthorized(err) => Self::Unauthorized(err.into()),
            Error::Reqwest(err) => {
                error!(?err);
                Self::Internal("failed to fetch".into())
            }
            Error::JsonWebToken(err) => {
                error!(?err);
                Self::Internal("failed to validate JWT".into())
            }
        }
    }
}
