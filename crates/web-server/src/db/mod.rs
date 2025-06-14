#![allow(async_fn_in_trait)]

mod model;
mod postgres;

pub use model::*;
pub use postgres::PostgresDb;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("{0} operation is unsupported")]
    Unsupported(&'static str),
}

pub trait AuthDb: Send + Sync {
    async fn find_invitation_token(&self, token: &[u8]) -> Result<InvitationToken, DbError>;
    async fn add_user(&self, user: &User, token_id: Uuid) -> Result<(), DbError>;
    async fn find_user_by_username(&self, username: &[u8]) -> Result<User, DbError>;
    async fn add_session(&self, session: &Session) -> Result<(), DbError>;
    async fn session(&self, session_id: Uuid) -> Result<Session, DbError>;
    async fn remove_user(&self, user_id: Uuid) -> Result<(), DbError>;
}

pub trait OperationsDb: Send + Sync {
    async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, DbError>;
    async fn add_operations(&self, operations: &[Operation]) -> Result<(), DbError>;
}
