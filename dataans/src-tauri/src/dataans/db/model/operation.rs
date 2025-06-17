use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, SqlitePool, SqliteTransaction, Transaction};
use time::serde::rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::db::sqlite::SqliteDb;
use crate::dataans::db::{DbError, File, Note, OperationDb, Space};
use crate::dataans::sync::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

    pub async fn apply(
        &self,
        operation_time: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        match self {
            Operation::CreateNote(note) => {
                SqliteDb::add_note(note.as_ref(), operation_time, transaction).await?;
            }
            Operation::UpdateNote(note) => {
                let local_note = SqliteDb::note_by_id(note.id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::update_note(note.as_ref(), operation_time, transaction).await?;
                }
            }
            Operation::DeleteNote(id) => {
                let local_note = SqliteDb::note_by_id(*id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::remove_note_inner(*id, operation_time, transaction).await?;
                }
            }
            Operation::CreateFile(file) => {
                SqliteDb::add_file(file.as_ref(), operation_time, transaction).await?;
            }
            Operation::DeleteFile(id) => {
                let local_file = SqliteDb::file_by_id(*id, transaction.as_mut()).await?;

                if local_file.updated_at < operation_time {
                    SqliteDb::remove_file(*id, operation_time, transaction).await?;
                }
            }
            Operation::CreateSpace(space) => {
                SqliteDb::add_space(space.as_ref(), operation_time, transaction).await?;
            }
            Operation::UpdateSpace(space) => {
                let local_space = SqliteDb::space_by_id(space.id, transaction.as_mut()).await?;

                if local_space.updated_at < operation_time {
                    SqliteDb::update_space(space.as_ref(), operation_time, transaction).await?;
                }
            }
            Operation::DeleteSpace(id) => {
                let local_space = SqliteDb::space_by_id(*id, transaction.as_mut()).await?;

                if local_space.updated_at < operation_time {
                    SqliteDb::remove_space(*id, operation_time, transaction).await?;
                }
            }
            Operation::SetNoteFiles(note_id, files) => {
                let local_note = SqliteDb::note_by_id(*note_id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::set_note_files(*note_id, files.as_ref(), operation_time, transaction).await?;
                }
            }
        }

        Ok(())
    }
}

impl Hash for Operation<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);

        match self {
            Operation::CreateNote(note) => note.hash(state),
            Operation::UpdateNote(note) => note.hash(state),
            Operation::DeleteNote(id) => id.hash(state),
            Operation::CreateFile(file) => file.hash(state),
            Operation::DeleteFile(id) => id.hash(state),
            Operation::CreateSpace(space) => space.hash(state),
            Operation::UpdateSpace(space) => space.hash(state),
            Operation::DeleteSpace(id) => id.hash(state),
            Operation::SetNoteFiles(note_id, files) => {
                note_id.hash(state);
                files.as_ref().hash(state);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationRecord<'data> {
    pub id: Uuid,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    pub operation: Operation<'data>,
}

pub type OperationRecordOwned = OperationRecord<'static>;

impl Hash for OperationRecord<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.created_at.hash(state);
        self.operation.hash(state);
    }
}

#[derive(FromRow)]
struct PlainOperationRecord {
    pub id: Uuid,
    pub created_at: OffsetDateTime,
    pub operation: String,
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
            now: OffsetDateTime::now_utc(),
            operation,
            transaction: self.pool.begin().await?,
        })
    }

    async fn log(operation: &PlainOperationRecord, transaction: &mut Transaction<'_, Sqlite>) -> Result<(), DbError> {
        let PlainOperationRecord {
            id,
            created_at,
            operation,
        } = operation;

        sqlx::query("INSERT INTO operations (id, created_at, operation) VALUES (?1, ?2, ?3)")
            .bind(id)
            .bind(created_at)
            .bind(operation)
            .execute(&mut **transaction)
            .await?;

        Ok(())
    }
}

impl OperationDb for OperationLogger {
    async fn operations(&self) -> Result<Vec<OperationRecordOwned>, DbError> {
        let mut connection = self.pool.acquire().await?;

        let operations: Vec<PlainOperationRecord> = sqlx::query_as("SELECT id, created_at, operation FROM operations")
            .fetch_all(&mut *connection)
            .await?;

        let operations = operations
            .into_iter()
            .map(|op| {
                let operation: OperationOwned = serde_json::from_str(&op.operation)?;

                Result::<_, DbError>::Ok(OperationRecord {
                    id: op.id,
                    created_at: op.created_at,
                    operation,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(operations)
    }

    async fn apply_operations(&self, operations: &[&OperationRecord<'_>]) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        for operation in operations {
            let OperationRecord {
                id,
                created_at,
                operation,
            } = operation;

            operation.apply(*created_at, &mut transaction).await?;
            OperationLogger::log(
                &PlainOperationRecord {
                    id: *id,
                    created_at: *created_at,
                    operation: serde_json::to_string(operation)?,
                },
                &mut transaction,
            )
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}

pub struct OperationLoggerGuard<'a> {
    now: OffsetDateTime,
    operation: Operation<'a>,
    transaction: SqliteTransaction<'a>,
}

impl<'a> OperationLoggerGuard<'a> {
    pub fn transaction(&mut self) -> &mut SqliteTransaction<'a> {
        &mut self.transaction
    }

    pub fn now(&self) -> OffsetDateTime {
        self.now
    }

    pub async fn commit(self) -> Result<(), DbError> {
        let OperationLoggerGuard {
            now,
            operation,
            mut transaction,
        } = self;

        OperationLogger::log(
            &PlainOperationRecord {
                id: Uuid::new_v4(),
                created_at: now,
                operation: serde_json::to_string(&operation)?,
            },
            &mut transaction,
        )
        .await?;
        transaction.commit().await?;

        Ok(())
    }
}

impl Deref for OperationLoggerGuard<'_> {
    type Target = <Sqlite as sqlx::Database>::Connection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.transaction.deref()
    }
}

impl DerefMut for OperationLoggerGuard<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.transaction.deref_mut()
    }
}
