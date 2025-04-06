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

pub trait SpaceDb: Send + Sync {
    async fn space(&self, space_id: Uuid) -> Result<Space, DbError>;
    async fn user_spaces(&self, user_id: Uuid) -> Result<Vec<Space>, DbError>;
    async fn add_space(&self, space: &Space) -> Result<(), DbError>;
    async fn update_space(&self, space: &Space) -> Result<(), DbError>;
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError>;
}

pub trait NoteDb: Send + Sync {
    async fn notes(&self, note_ids: &[Uuid]) -> Result<Vec<Note>, DbError>;
    async fn add_note(&self, note: &Note) -> Result<Uuid, DbError>;
    async fn update_note(&self, note: &Note) -> Result<(), DbError>;
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError>;
    async fn note_owner(&self, note_id: Uuid) -> Result<Uuid, DbError>;
}

pub trait SyncDb: Send + Sync {
    async fn blocks(&self, space_id: Uuid) -> Result<Vec<SyncBlock>, DbError>;
    async fn block_notes(&self, block_id: Uuid) -> Result<Vec<NoteChecksum>, DbError>;
    /// Returns the id of the user who owns the block.
    async fn block_owner(&self, block_id: Uuid) -> Result<Uuid, DbError>;
}
