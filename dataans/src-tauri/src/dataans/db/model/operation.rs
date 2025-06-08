use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error as SqlxError, FromRow, Row, Sqlite, SqlitePool, SqliteTransaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::db::{DbError, File, Note, Space};

#[derive(Debug, PartialEq, Eq)]
pub enum Operation<'data> {
    CreateNote(Cow<'data, Note>),
    UpdateNote(Cow<'data, Note>),
    DeleteNote(Uuid),
    CreateFile(Cow<'data, File>),
    DeleteFile(Uuid),
    CreateSpace(Cow<'data, Space>),
    UpdateSpace(Cow<'data, Space>),
    DeleteSpace(Uuid),
    SetNoteFiles(Uuid, Cow<'data, [Uuid]>),
}

pub type OperationOwned = Operation<'static>;

impl Operation<'_> {
    pub fn name(&self) -> &str {
        match self {
            Operation::CreateNote(_) => "CreateNote",
            Operation::UpdateNote(_) => "UpdateNote",
            Operation::DeleteNote(_) => "DeleteNote",
            Operation::CreateFile(_) => "CreateFile",
            Operation::DeleteFile(_) => "DeleteFile",
            Operation::CreateSpace(_) => "CreateSpace",
            Operation::UpdateSpace(_) => "UpdateSpace",
            Operation::DeleteSpace(_) => "DeleteSpace",
            Operation::SetNoteFiles(_, _) => "SetNoteFiles",
        }
    }

    pub fn data(&self) -> Result<String, DbError> {
        Ok(match self {
            Operation::CreateNote(note) => serde_json::to_string(note)?,
            Operation::UpdateNote(note) => serde_json::to_string(note)?,
            Operation::DeleteNote(id) => serde_json::to_string(id)?,
            Operation::CreateFile(file) => serde_json::to_string(file)?,
            Operation::DeleteFile(id) => serde_json::to_string(id)?,
            Operation::CreateSpace(space) => serde_json::to_string(space)?,
            Operation::UpdateSpace(space) => serde_json::to_string(space)?,
            Operation::DeleteSpace(id) => serde_json::to_string(id)?,
            Operation::SetNoteFiles(note_id, files) => serde_json::to_string(&(note_id, files.as_ref()))?,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for OperationOwned {
    fn from_row(row: &SqliteRow) -> Result<Self, SqlxError> {
        let name: String = row.try_get("name")?;
        let data: String = row.try_get("operation")?;

        let operation = match name.as_str() {
            "CreateNote" => {
                let note: Note = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::CreateNote(Cow::Owned(note))
            }
            "UpdateNote" => {
                let note: Note = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::UpdateNote(Cow::Owned(note))
            }
            "DeleteNote" => {
                let id: Uuid = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::DeleteNote(id)
            }
            "CreateFile" => {
                let file: File = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::CreateFile(Cow::Owned(file))
            }
            "DeleteFile" => {
                let id: Uuid = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::DeleteFile(id)
            }
            "CreateSpace" => {
                let space: Space = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::CreateSpace(Cow::Owned(space))
            }
            "UpdateSpace" => {
                let space: Space = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::UpdateSpace(Cow::Owned(space))
            }
            "DeleteSpace" => {
                let id: Uuid = serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::DeleteSpace(id)
            }
            "SetNoteFiles" => {
                let (note_id, files): (Uuid, Vec<Uuid>) =
                    serde_json::from_str(&data).map_err(|err| SqlxError::Decode(Box::new(err)))?;
                Operation::SetNoteFiles(note_id, Cow::Owned(files))
            }
            _ => {
                return Err(SqlxError::Decode(
                    format!("operation is not supported: {}", name).into(),
                ))
            }
        };

        Ok(operation)
    }
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct OperationRecord<'data> {
    pub id: Uuid,
    pub timestamp: OffsetDateTime,
    pub operation: Operation<'data>,
}

pub struct OperationLogger {
    pool: SqlitePool,
}

impl OperationLogger {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn read_only_connection(&self) -> Result<PoolConnection<Sqlite>, DbError> {
        Ok(self.pool.acquire().await?)
    }

    pub async fn begin<'a>(&self, operation: Operation<'a>) -> Result<OperationLoggerGuard<'a>, DbError> {
        Ok(OperationLoggerGuard {
            operation,
            transaction: self.pool.begin().await?,
        })
    }
}

pub struct OperationLoggerGuard<'a> {
    operation: Operation<'a>,
    transaction: SqliteTransaction<'a>,
}

impl<'a> OperationLoggerGuard<'a> {
    pub fn transaction(&mut self) -> &mut SqliteTransaction<'a> {
        &mut self.transaction
    }

    pub async fn commit(self) -> Result<(), DbError> {
        let OperationLoggerGuard {
            operation,
            mut transaction,
        } = self;

        sqlx::query("INSERT INTO operations (id, created_at, name, operation) VALUES (?1, ?2, ?3, ?4)")
            .bind(Uuid::new_v4())
            .bind(OffsetDateTime::now_utc())
            .bind(operation.name())
            .bind(operation.data()?)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl<'a> Deref for OperationLoggerGuard<'a> {
    type Target = <Sqlite as sqlx::Database>::Connection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.transaction.deref()
    }
}

impl<'a> DerefMut for OperationLoggerGuard<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.transaction.deref_mut()
    }
}
