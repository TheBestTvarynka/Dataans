pub mod model;
pub mod sqlite;

use common::event::DataEvent;
use thiserror::Error;
use uuid::Uuid;

pub use self::model::*;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("sqlx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

// TODO: split into separate traits (see the `web-server` crate).
pub trait Db: Send + Sync {
    #[allow(dead_code)]
    async fn files(&self) -> Result<Vec<File>, DbError>;
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError>;
    async fn add_file(&self, file: &File) -> Result<(), DbError>;
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError>;

    async fn spaces(&self) -> Result<Vec<Space>, DbError>;
    async fn space_by_id(&self, space_id: Uuid) -> Result<Space, DbError>;
    async fn create_space(&self, space: &Space) -> Result<(), DbError>;
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError>;
    async fn update_space(&self, space: &Space) -> Result<(), DbError>;

    async fn notes(&self) -> Result<Vec<Note>, DbError>;
    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError>;
    async fn note_by_id(&self, note_id: Uuid) -> Result<Note, DbError>;
    async fn create_note(&self, note: &Note) -> Result<(), DbError>;
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError>;
    async fn update_note(&self, note: &Note) -> Result<(), DbError>;
    async fn note_files(&self, note_id: Uuid) -> Result<Vec<File>, DbError>;
    async fn set_note_files(&self, note_id: Uuid, files: &[Uuid]) -> Result<(), DbError>;
}

pub trait OperationDb: Send + Sync {
    async fn operations(&self) -> Result<Vec<OperationRecordOwned>, DbError>;
    async fn apply_operation(&self, operations: &OperationRecord<'_>) -> Result<Option<DataEvent>, DbError>;
    async fn files(&self) -> Result<Vec<File>, DbError>;
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError>;
    async fn mark_file_as_uploaded(&self, file_id: Uuid) -> Result<(), DbError>;
}
