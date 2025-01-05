#![allow(async_fn_in_trait)]

mod model;
mod postgres;

pub use model::*;
pub use postgres::PostgresDb;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

pub trait AuthDb: Send + Sync {
    async fn find_invitation_token(&self, token: &[u8]) -> Result<InvitationToken, DbError>;
    async fn add_user(&self, user: &User, token_id: Uuid) -> Result<(), DbError>;
}
