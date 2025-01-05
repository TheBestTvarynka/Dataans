mod model;
mod postgres;

pub use postgres::PostgresDb;
use thiserror::Error;
use uuid::Uuid;

use self::model::*;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

pub trait AuthDb {
    async fn find_invitation_token(&self, token: &[u8]) -> Result<Option<InvitationToken>, DbError>;
    async fn add_user(&self, user: &User) -> Result<(), DbError>;
    async fn assign_invitation_token(&self, token_id: Uuid, user_id: Uuid) -> Result<(), DbError>;
}
