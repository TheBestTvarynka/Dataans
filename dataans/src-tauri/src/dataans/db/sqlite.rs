use std::borrow::Cow;

use sqlx::{SqlitePool, Transaction};
use uuid::Uuid;

use super::*;

const NOTE_FILES: &str = "SELECT files.id, files.name, files.path, files.is_deleted
    FROM files
        LEFT JOIN notes_files ON files.id = notes_files.file_id
    WHERE notes_files.note_id = ?1 AND files.is_deleted = FALSE";

pub struct SqliteDb {
    pool: OperationLogger,
}

impl SqliteDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: OperationLogger::new(pool),
        }
    }
}

impl SqliteDb {
    #[instrument(ret, skip(transaction))]
    async fn remove_note_inner(note_id: Uuid, transaction: &mut Transaction<'_, sqlx::Sqlite>) -> Result<(), DbError> {
        let note_files: Vec<File> = sqlx::query_as(NOTE_FILES)
            .bind(note_id)
            .fetch_all(&mut **transaction)
            .await?;

        // We keep such records, because we implement soft deletion. There is not need to delete such records.
        // sqlx::query("DELETE FROM notes_files WHERE note_id = ?1")
        //     .bind(note_id)
        //     .execute(&mut **transaction)
        //     .await?;

        // TODO: replace with `join_all`.
        for file in note_files {
            sqlx::query!("UPDATE files SET is_deleted = TRUE WHERE id = ?1", file.id)
                .execute(&mut **transaction)
                .await?;
        }

        sqlx::query!("UPDATE notes SET is_deleted = TRUE WHERE id = ?1", note_id)
            .execute(&mut **transaction)
            .await?;

        Ok(())
    }
}

impl Db for SqliteDb {
    #[instrument(ret, skip(self))]
    async fn files(&self) -> Result<Vec<File>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as("SELECT id, name, path, is_deleted FROM files WHERE is_deleted = FALSE")
            .fetch_all(&mut *connection)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as("SELECT id, name, path, is_deleted FROM files WHERE id = ?1 AND is_deleted = FALSE")
            .bind(file_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn add_file(&self, file: &File) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateFile(Cow::Borrowed(file))).await?;

        let File {
            id,
            name,
            path,
            // We explicitly ignore `is_deleted` because it is always TRUE for new files.
            is_deleted: _,
        } = file;

        sqlx::query!("INSERT INTO files (id, name, path) VALUES (?1, ?2, ?3)", id, name, path,)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteFile(file_id)).await?;

        sqlx::query!("UPDATE files SET is_deleted = TRUE WHERE id = ?1", file_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn spaces(&self) -> Result<Vec<Space>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let spaces = sqlx::query_as(
            "SELECT id, name, avatar_id, created_at, updated_at, is_deleted FROM spaces WHERE is_deleted = FALSE",
        )
        .fetch_all(&mut *connection)
        .await?;

        Ok(spaces)
    }

    #[instrument(ret, skip(self))]
    async fn space_by_id(&self, space_id: Uuid) -> Result<Space, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let space = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_deleted FROM spaces WHERE id = ?1 AND is_deleted = FALSE")
            .bind(space_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(space)
    }

    #[instrument(ret, skip(self))]
    async fn create_space(&self, space: &Space) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateSpace(Cow::Borrowed(space))).await?;

        let Space {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            // We explicitly ignore `is_deleted` because it is always FALSE for new spaces.
            is_deleted: _,
        } = space;

        sqlx::query!(
            "INSERT INTO spaces (id, name, avatar_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteSpace(space_id)).await?;

        let notes: Vec<Note> =
            sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE space_id = ?1 AND is_deleted = FALSE")
                .bind(space_id)
                .fetch_all(&mut *transaction)
                .await?;

        // TODO: replace with `join_all`.
        for note in notes {
            SqliteDb::remove_note_inner(note.id, transaction.transaction()).await?;
        }

        let space: Space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_deleted FROM spaces WHERE id = ?1 AND is_deleted = FALSE")
                .bind(space_id)
                .fetch_one(&mut *transaction)
                .await?;

        sqlx::query!("UPDATE spaces SET is_deleted = TRUE WHERE id = ?1", space_id)
            .execute(&mut *transaction)
            .await?;

        sqlx::query!("UPDATE files SET is_deleted = true WHERE id = ?1", space.avatar_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn update_space(&self, space: &Space) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::UpdateSpace(Cow::Borrowed(space))).await?;

        let Space {
            id,
            name,
            avatar_id,
            created_at: _,
            updated_at,
            // We explicitly ignore `is_deleted` because it must be updated only on deletion.
            is_deleted: _,
        } = space;

        sqlx::query!(
            "UPDATE spaces SET name = ?1, avatar_id = ?2, updated_at = ?3 WHERE id = ?4",
            name,
            avatar_id,
            updated_at,
            id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn notes(&self) -> Result<Vec<Note>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let notes = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE is_deleted = FALSE",
        )
        .fetch_all(&mut *connection)
        .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let notes = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE space_id = ?1 AND is_deleted = FALSE")
            .bind(space_id)
            .fetch_all(&mut *connection)
            .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn note_by_id(&self, note_id: Uuid) -> Result<Note, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let note = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE id = ?1 AND is_deleted = FALSE")
            .bind(note_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(note)
    }

    #[instrument(ret, skip(self))]
    async fn create_note(&self, note: &Note) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateNote(Cow::Borrowed(note))).await?;

        let Note {
            id,
            text,
            created_at,
            updated_at,
            space_id,
            // We explicitly ignore `is_deleted` because it is always FALSE for new notes.
            is_deleted: _,
        } = note;

        sqlx::query!(
            "INSERT INTO notes (id, text, created_at,  updated_at, space_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            text,
            created_at,
            updated_at,
            space_id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteNote(note_id)).await?;

        SqliteDb::remove_note_inner(note_id, transaction.transaction()).await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::UpdateNote(Cow::Borrowed(note))).await?;

        let Note {
            id,
            text,
            created_at: _,
            updated_at,
            space_id: _,
            // We explicitly ignore `is_deleted` because it must be updated only on deletion.
            is_deleted: _,
        } = note;

        sqlx::query!(
            "UPDATE notes SET text = ?1, updated_at = ?2 WHERE id = ?3",
            text,
            updated_at,
            id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn note_files(&self, note_id: Uuid) -> Result<Vec<File>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as(NOTE_FILES)
            .bind(note_id)
            .fetch_all(&mut *connection)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn set_note_files(&self, note_id: Uuid, files: &[Uuid]) -> Result<(), DbError> {
        let mut transaction = self
            .pool
            .begin(Operation::SetNoteFiles(note_id, Cow::Borrowed(files)))
            .await?;

        sqlx::query("DELETE FROM notes_files WHERE note_id = ?1")
            .bind(note_id)
            .execute(&mut *transaction)
            .await?;

        for file_id in files {
            sqlx::query("INSERT INTO notes_files (note_id, file_id) VALUES (?1, ?2)")
                .bind(note_id)
                .bind(file_id)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}

impl OperationDb for SqliteDb {
    async fn operations(&self) -> Result<Vec<OperationRecordOwned>, DbError> {
        #[derive(sqlx::FromRow)]
        struct PlainOperationRecord {
            pub id: Uuid,
            pub created_at: time::OffsetDateTime,
            pub name: String,
            pub operation: String,
        }

        let mut connection = self.pool.read_only_connection().await?;

        let operations: Vec<PlainOperationRecord> =
            sqlx::query_as("SELECT id, created_at, name, operation FROM operations")
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

    async fn add_operations(&self, operations: &[OperationRecord<'_>]) -> Result<(), DbError> {
        todo!()
    }
}
