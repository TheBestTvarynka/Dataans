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

    #[error("{0} operation is unsupported")]
    Unsupported(&'static str),
}

pub trait AuthDb: Send + Sync {
    async fn find_invitation_token(&self, token: &[u8]) -> Result<InvitationToken, DbError>;
    async fn add_user(&self, user: &User, token_id: Uuid) -> Result<(), DbError>;
    async fn find_user_by_username(&self, username: &[u8]) -> Result<User, DbError>;
    async fn add_session(&self, session: &Session) -> Result<(), DbError>;
}

pub trait SpaceDb: Send + Sync {
    async fn add_space(&self, space: &Space) -> Result<(), DbError>;
    async fn update_space(&self, space: &Space) -> Result<(), DbError>;
    async fn delete_space(&self, space_id: Uuid) -> Result<(), DbError>;
}

pub trait NoteDb: Send + Sync {
    async fn add_note(&self, note: &Note) -> Result<(), DbError>;
    async fn update_note(&self, note: &Note) -> Result<(), DbError>;
    async fn delete_note(&self, note_id: Uuid) -> Result<(), DbError>;
}
