mod model;
mod sqlite;

use thiserror::Error;

use self::model::*;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("slx error: {0:?}")]
    SqlxError(#[from] sqlx::Error),
}

pub trait Db {
    async fn files(&self) -> Result<Vec<File>, DbError>;
    async fn add_file(&self, file: &File) -> Result<(), DbError>;

    async fn spaces(&self) -> Result<Vec<Space>, DbError>;
    async fn create_space(&self, space: &Space) -> Result<(), DbError>;
}
