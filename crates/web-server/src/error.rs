use thiserror::Error;

use crate::db::DbError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DbError: {0:?}")]
    DbError(DbError),

    #[error("the requested resource not found")]
    NotFound,

    #[error("cannot hash password: {0}")]
    Argon2Hash(#[from] argon2::password_hash::Error),

    #[error("cannot parse password hash")]
    PasswordHashParsingError,

    #[error("encryption error: {0}")]
    Encryption(#[from] aes_gcm::Error),

    #[error("invalid encryption key or IV length")]
    InvalidKeyLength,

    #[error("failed to decrypt the data: {0}")]
    DecryptionFailed(&'static str),

    #[error("session error: {0}")]
    Session(&'static str),

    #[error("access denied")]
    AccessDenied,

    #[error("invalid {0}")]
    InvalidData(&'static str),

    #[error("internal error: {0}")]
    Internal(&'static str),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
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
            Error::Argon2Hash(err) => {
                error!(?err);

                if let argon2::password_hash::Error::Password = err {
                    Self::InvalidCredentials("invalid credentials".into())
                } else {
                    Self::PasswordHashingError("failed to hash password".into())
                }
            }
            Error::Encryption(err) => {
                error!(?err);
                Self::Internal("internal error".into())
            }
            Error::Internal(err) => {
                error!(err);
                Self::Internal("internal error".into())
            }
            Error::InvalidKeyLength => Self::Internal("internal error".into()),
            Error::PasswordHashParsingError => Self::PasswordHashingError("unable to verify credentials".into()),
            Error::AccessDenied => Self::AccessDenied("access denied".into()),
            Error::InvalidData(_) => Self::InvalidData(error.to_string()),
            Error::DecryptionFailed(reason) => Self::Internal(reason.into()),
            Error::Io(err) => Self::Internal(format!("{err:?}")),
            Error::Session(_) => Self::Unauthorized(error.to_string()),
        }
    }
}
