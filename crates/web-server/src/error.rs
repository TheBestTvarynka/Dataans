use thiserror::Error;
use web_api_types::InvitationToken;

use crate::db::DbError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DbError: {0:?}")]
    DbError(#[from] DbError),

    #[error("Invitation token not found")]
    InvitationTokenNotFound(InvitationToken),

    #[error("Cannot hash password: {0:?}")]
    Argon2(#[from] argon2::password_hash::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for web_api_types::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::DbError(err) => {
                error!(?err);
                Self::DbError("Interaction with the database failed".into())
            }
            Error::InvitationTokenNotFound(token) => Self::InvitationTokenNotFound(token.into_inner()),
            Error::Argon2(err) => {
                error!(?err);
                Self::PasswordHashingError("Failed to hash password".into())
            }
        }
    }
}
