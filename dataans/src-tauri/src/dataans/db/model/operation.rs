//! User's operations logging and applying.
//!
//! Alongside user's data, the app also tracks all user operations like note creation,
//! updating, space deletion, etc. During the sync process all these operations are
//! synchronized.
//!
//! This module provides operations reading, writing, and applying.

use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use common::event::DataEvent;
use common::note::{File as EventFile, FileStatus, Id as NoteId, MdText, Note as EventNote};
use common::space::{Avatar, Id as SpaceId, Name as SpaceName, Space as EventSpace};
use common::{CreationDate, UpdateDate};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, SqlitePool, SqliteTransaction, Transaction};
use time::OffsetDateTime;
use time::serde::rfc3339;
use uuid::Uuid;

use crate::dataans::db::sqlite::SqliteDb;
use crate::dataans::db::{DbError, File, Note, OperationDb, Space};
use crate::dataans::sync::{Hash, Hasher};

/// The user's operation type (and its data).
///
/// This enumeration lists all possible user operation types.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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
    /// Returns operation name.
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

    /// Applies the operation on the local database.
    ///
    /// Returns the [DataEvent] that can be optionally sent, for example, to the frontend
    /// to inform about new changes.
    ///
    /// There can be conflicts during the operation applying. The current strategy is
    /// last write wins.
    pub async fn apply(
        &self,
        operation_time: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<Option<DataEvent>, DbError> {
        let event = match self {
            Operation::CreateNote(note) => {
                SqliteDb::add_note(note.as_ref(), operation_time, transaction).await?;

                let Note {
                    id,
                    text,
                    created_at,
                    updated_at,
                    space_id,
                    is_deleted: _,
                } = note.as_ref();

                let files = SqliteDb::note_files(*id, transaction.as_mut())
                    .await?
                    .into_iter()
                    .map(|file| {
                        let File {
                            id,
                            name,
                            path,
                            created_at: _,
                            updated_at: _,
                            is_deleted: _,
                            is_uploaded,
                        } = file;

                        let path = PathBuf::from(path);
                        let status = FileStatus::status_for_file(&path, is_uploaded);

                        EventFile {
                            id: id.into(),
                            name,
                            path,
                            status,
                        }
                    })
                    .collect();

                Some(DataEvent::NoteAdded(EventNote {
                    id: NoteId::from(*id),
                    text: MdText::from(text.clone()),
                    created_at: CreationDate::from(*created_at),
                    updated_at: UpdateDate::from(*updated_at),
                    space_id: SpaceId::from(*space_id),
                    files,
                }))
            }
            Operation::UpdateNote(note) => {
                let local_note = SqliteDb::absolute_note_by_id(note.id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::update_note(note.as_ref(), operation_time, transaction).await?;

                    let Note {
                        id,
                        text,
                        created_at,
                        updated_at,
                        space_id,
                        is_deleted: _,
                    } = note.as_ref();

                    let files = SqliteDb::note_files(*id, transaction.as_mut())
                        .await?
                        .into_iter()
                        .map(|file| {
                            let File {
                                id,
                                name,
                                path,
                                created_at: _,
                                updated_at: _,
                                is_deleted: _,
                                is_uploaded,
                            } = file;

                            let path = PathBuf::from(path);
                            let status = FileStatus::status_for_file(&path, is_uploaded);

                            EventFile {
                                id: id.into(),
                                name,
                                path,
                                status,
                            }
                        })
                        .collect();

                    Some(DataEvent::NoteUpdated(EventNote {
                        id: NoteId::from(*id),
                        text: MdText::from(text.clone()),
                        created_at: CreationDate::from(*created_at),
                        updated_at: UpdateDate::from(*updated_at),
                        space_id: SpaceId::from(*space_id),
                        files,
                    }))
                } else {
                    None
                }
            }
            Operation::DeleteNote(id) => {
                let local_note = SqliteDb::absolute_note_by_id(*id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::remove_note_inner(*id, operation_time, transaction).await?;

                    Some(DataEvent::NoteDeleted(
                        SpaceId::from(local_note.space_id),
                        NoteId::from(local_note.id),
                    ))
                } else {
                    None
                }
            }
            Operation::CreateFile(file) => {
                let mut file = file.clone().into_owned();
                file.is_uploaded = true;

                SqliteDb::add_file(&file, operation_time, transaction).await?;

                let File {
                    id,
                    name,
                    path,
                    created_at: _,
                    updated_at: _,
                    is_deleted: _,
                    is_uploaded,
                } = file;

                let path = PathBuf::from(path);
                let status = FileStatus::status_for_file(&path, is_uploaded);

                Some(DataEvent::FileAdded(EventFile {
                    id: id.into(),
                    name,
                    path,
                    status,
                }))
            }
            Operation::DeleteFile(id) => {
                let local_file = SqliteDb::absolute_file_by_id(*id, transaction.as_mut()).await?;

                if local_file.updated_at < operation_time {
                    SqliteDb::remove_file(*id, operation_time, transaction).await?;
                }

                None
            }
            Operation::CreateSpace(space) => {
                SqliteDb::add_space(space.as_ref(), operation_time, transaction).await?;

                let Space {
                    id,
                    name,
                    avatar_id,
                    created_at,
                    updated_at,
                    is_deleted: _,
                } = space.as_ref();

                let avatar = SqliteDb::file_by_id(*avatar_id, transaction.as_mut()).await?;

                Some(DataEvent::SpaceAdded(EventSpace {
                    id: SpaceId::from(*id),
                    name: SpaceName::from(name.clone()),
                    created_at: CreationDate::from(*created_at),
                    updated_at: UpdateDate::from(*updated_at),
                    avatar: Avatar::new((*avatar_id).into(), avatar.path),
                }))
            }
            Operation::UpdateSpace(space) => {
                let local_space = SqliteDb::absolute_space_by_id(space.id, transaction.as_mut()).await?;

                if local_space.updated_at < operation_time {
                    SqliteDb::update_space(space.as_ref(), operation_time, transaction).await?;

                    let Space {
                        id,
                        name,
                        avatar_id,
                        created_at,
                        updated_at,
                        is_deleted: _,
                    } = space.as_ref();

                    let avatar = SqliteDb::file_by_id(*avatar_id, transaction.as_mut()).await?;

                    Some(DataEvent::SpaceUpdated(EventSpace {
                        id: SpaceId::from(*id),
                        name: SpaceName::from(name.clone()),
                        created_at: CreationDate::from(*created_at),
                        updated_at: UpdateDate::from(*updated_at),
                        avatar: Avatar::new((*avatar_id).into(), avatar.path),
                    }))
                } else {
                    None
                }
            }
            Operation::DeleteSpace(id) => {
                let local_space = SqliteDb::absolute_space_by_id(*id, transaction.as_mut()).await?;

                if local_space.updated_at < operation_time {
                    SqliteDb::remove_space(*id, operation_time, transaction).await?;

                    Some(DataEvent::SpaceDeleted(SpaceId::from(local_space.id)))
                } else {
                    None
                }
            }
            Operation::SetNoteFiles(note_id, files) => {
                let local_note = SqliteDb::note_by_id(*note_id, transaction.as_mut()).await?;

                if local_note.updated_at < operation_time {
                    SqliteDb::set_note_files(*note_id, files.as_ref(), operation_time, transaction).await?;

                    let Note {
                        id,
                        text,
                        created_at,
                        updated_at,
                        space_id,
                        is_deleted: _,
                    } = local_note;

                    let files = SqliteDb::note_files(id, transaction.as_mut())
                        .await?
                        .into_iter()
                        .map(|file| {
                            let File {
                                id,
                                name,
                                path,
                                created_at: _,
                                updated_at: _,
                                is_deleted: _,
                                is_uploaded,
                            } = file;

                            let path = PathBuf::from(path);
                            let status = FileStatus::status_for_file(&path, is_uploaded);

                            EventFile {
                                id: id.into(),
                                name,
                                path,
                                status,
                            }
                        })
                        .collect();

                    Some(DataEvent::NoteUpdated(EventNote {
                        id: NoteId::from(id),
                        text: MdText::from(text),
                        created_at: CreationDate::from(created_at),
                        updated_at: UpdateDate::from(updated_at),
                        space_id: SpaceId::from(space_id),
                        files,
                    }))
                } else {
                    None
                }
            }
        };

        Ok(event)
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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

/// Operation record used to store [OperationRecord] in the local DB.
#[derive(FromRow)]
struct PlainOperationRecord {
    pub id: Uuid,
    pub created_at: OffsetDateTime,
    /// JSON string: serialized [OperationRecord].
    pub operation: String,
}

/// This is a spacial database pool wrapper used to write user's operations
/// automatically.
///
/// As already said, the app tracks all user operations. It needs to record
/// all database changes. If the developer did it manually, then it would be
/// super easy to forget to insert a new operation when making changes in the
/// local DB. We do not want it.
///
/// It is impossible to forget to do it because of the help of this wrapper.
/// The only way to change the local database is to request a transaction using
/// the [OperationLogger::begin] method. It returns the OperationLoggerGuard,
/// which encapsulates the sqlx transaction and will automatically insert a new
/// operation during transaction committing.
///
/// Unfortunately, it is possible to overcome this and use the [OperationLogger::read_only_connection]
/// method to modify the local database directly. It is the developer's responsibility
/// to use the connection returned from the [OperationLogger::read_only_connection] method
/// only for read-only purposes.
pub struct OperationLogger {
    pool: SqlitePool,
}

impl OperationLogger {
    /// Creates a new [OperationLogger] instance.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Returns `true` if the operation with the given id already exists in the local database.
    async fn is_operation_exists(id: Uuid, transaction: &mut Transaction<'_, Sqlite>) -> Result<bool, DbError> {
        let record: (i64,) = sqlx::query_as("SELECT COUNT(id) FROM operations WHERE id = ?1")
            .bind(id)
            .fetch_one(&mut **transaction)
            .await?;

        Ok(record.0 > 0)
    }

    /// Returns the direct connection to the database.
    ///
    /// # Correctness
    ///
    /// It is the developer's responsibility to use the returned connection only for read-only purposes.
    /// Performing writes may result in broken data synchronization feature.
    pub async fn read_only_connection(&self) -> Result<PoolConnection<Sqlite>, DbError> {
        Ok(self.pool.acquire().await?)
    }

    /// Begins a new transaction.
    ///
    /// The returned guard automatically inserts a new operation during transaction committing.
    pub async fn begin<'a>(&self, operation: Operation<'a>) -> Result<OperationLoggerGuard<'a>, DbError> {
        Ok(OperationLoggerGuard {
            now: OffsetDateTime::now_utc(),
            operation,
            transaction: self.pool.begin().await?,
        })
    }

    /// Writes a new operation into a local database.
    ///
    /// This function should never be exported and should never be used outside of this module.
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

    async fn apply_operation(&self, operation: &OperationRecord<'_>) -> Result<Option<DataEvent>, DbError> {
        let mut transaction = self.pool.begin().await?;

        let OperationRecord {
            id,
            created_at,
            operation,
        } = operation;

        // In theory, the following check is not needed. But in the past we has a bug in the synchronization
        // algorithm that caused the same operation to be applied multiple times. To be safe, we add this check.
        if Self::is_operation_exists(*id, &mut transaction).await? {
            warn!(
                "Operation ({id}) already exists, skipping... This should not be possible and should not happen (but it is what it is)."
            );
            trace!(?operation, "Operation ({id}) already exists, skipping...");

            transaction.rollback().await?;

            return Ok(None);
        }

        let event = operation.apply(*created_at, &mut transaction).await?;
        OperationLogger::log(
            &PlainOperationRecord {
                id: *id,
                created_at: *created_at,
                operation: serde_json::to_string(operation)?,
            },
            &mut transaction,
        )
        .await?;

        transaction.commit().await?;

        Ok(event)
    }

    async fn files(&self) -> Result<Vec<File>, DbError> {
        let mut connection = self.pool.acquire().await?;

        let files = SqliteDb::files(&mut connection).await?;

        Ok(files)
    }

    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let mut connection = self.pool.acquire().await?;

        let file = SqliteDb::file_by_id(file_id, &mut connection).await?;

        Ok(file)
    }

    async fn mark_file_as_uploaded(&self, file_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        SqliteDb::mark_file_as_uploaded(file_id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}

/// sqlx transaction wrapper for automatic user operation logging.
///
/// This guard automatically inserts a new operation during transaction committing.
pub struct OperationLoggerGuard<'a> {
    now: OffsetDateTime,
    operation: Operation<'a>,
    transaction: SqliteTransaction<'a>,
}

impl<'a> OperationLoggerGuard<'a> {
    /// Returns the inner sqlx transaction.
    pub fn transaction(&mut self) -> &mut SqliteTransaction<'a> {
        &mut self.transaction
    }

    /// Returns the [OffsetDateTime] of the current operation.
    ///
    /// Operation datetime is the [OperationLoggerGuard] creation datetime.
    pub fn now(&self) -> OffsetDateTime {
        self.now
    }

    /// Commits the transaction.
    ///
    /// A new operation will be automatically inserted in the local database.
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
