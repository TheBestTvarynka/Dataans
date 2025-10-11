mod model;
mod postgres;

pub use model::*;
pub use postgres::PostgresDb;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("{0} operation is unsupported")]
    Unsupported(&'static str),

    #[error("invalid operation: {0}")]
    InvalidOperation(&'static str),

    #[error("user already exist")]
    UserAlreadyExist,
}

/// User database interface.
///
/// The user's password is not actually a password. It is a hash of the encryption (secret) key.
/// This hash is used to verify that the user provided the correct password and salt for generating
/// the encryption key.
pub trait UserDb: Send + Sync {
    /// Initialize the app user.
    ///
    /// This should be called only once, when the user is signed in for the first time.
    async fn init(&self, user: &User) -> Result<(), DbError>;
    /// Returns the app user.
    ///
    /// If the user does not exist, returns an error.
    async fn user(&self) -> Result<User, DbError>;
}

/// Operations database interface.
pub trait OperationsDb: Send + Sync {
    /// Returns a list of operations, skipping the first `operations_to_skip` operations.
    ///
    /// The resulting operations are ordered by creation time.
    async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, DbError>;
    async fn add_operations(&self, operations: &[Operation]) -> Result<(), DbError>;
}
