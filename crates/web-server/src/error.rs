use thiserror::Error;

use crate::db::DbError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DbError: {0:?}")]
    DbError(#[from] DbError),
}

pub type Result<T> = std::result::Result<T, Error>;
