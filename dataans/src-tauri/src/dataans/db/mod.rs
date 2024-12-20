mod model;
mod sqlite;

use thiserror::Error;
use uuid::Uuid;

use self::model::*;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("slx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

pub trait Db {
    async fn files(&self) -> Result<Vec<File>, DbError>;
    async fn add_file(&self, file: &File) -> Result<(), DbError>;
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError>;
    async fn update_file(&self, file: &File) -> Result<(), DbError>;

    async fn spaces(&self) -> Result<Vec<Space>, DbError>;
    async fn create_space(&self, space: &Space) -> Result<(), DbError>;
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError>;
    async fn update_space(&self, space: &Space) -> Result<(), DbError>;

    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError>;
    async fn create_note(&self, note: &Note) -> Result<(), DbError>;
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError>;
    async fn update_note(&self, note: &Note) -> Result<(), DbError>;
}
