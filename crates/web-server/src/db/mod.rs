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
}

pub trait OperationsDb: Send + Sync {
    async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, DbError>;
    async fn add_operations(&self, operations: &[Operation]) -> Result<(), DbError>;
}
