mod postgres;

pub use postgres::PostgresDb;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

pub trait AuthDb {
    async fn create_user(&self) -> Result<(), DbError>;
}
