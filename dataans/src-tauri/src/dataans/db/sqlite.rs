use std::borrow::Cow;
use std::sync::Arc;

use sqlx::{Sqlite, SqliteConnection, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use super::*;

const NOTE_FILES: &str = "SELECT files.id, files.name, files.path, files.created_at, files.updated_at, files.is_deleted
    FROM files
        LEFT JOIN notes_files ON files.id = notes_files.file_id
    WHERE notes_files.note_id = ?1 AND files.is_deleted = FALSE";

pub struct SqliteDb {
    pool: Arc<OperationLogger>,
}

impl SqliteDb {
    pub fn new(pool: Arc<OperationLogger>) -> Self {
        Self { pool }
    }
}

impl SqliteDb {
    pub async fn remove_note_inner(
        note_id: Uuid,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
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
            sqlx::query!(
                "UPDATE files SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2",
                now,
                file.id
            )
            .execute(&mut **transaction)
            .await?;
        }

        sqlx::query!(
            "UPDATE notes SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2",
            now,
            note_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn space_by_id(space_id: Uuid, connection: &mut SqliteConnection) -> Result<Space, DbError> {
        let space = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_deleted FROM spaces WHERE id = ?1 AND is_deleted = FALSE")
            .bind(space_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(space)
    }

    pub async fn add_space(
        space: &Space,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), DbError> {
        let Space {
            id,
            name,
            avatar_id,
            created_at: _,
            updated_at: _,
            is_deleted: _,
        } = space;

        sqlx::query!(
            "INSERT INTO spaces (id, name, avatar_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            name,
            avatar_id,
            now,
            now,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn update_space(
        space: &Space,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        let Space {
            id,
            name,
            avatar_id,
            created_at: _,
            updated_at: _,
            is_deleted,
        } = space;

        sqlx::query!(
            "UPDATE spaces SET name = ?1, avatar_id = ?2, updated_at = ?3, is_deleted = ?4 WHERE id = ?5",
            name,
            avatar_id,
            now,
            is_deleted,
            id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn remove_space(
        space_id: Uuid,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        let notes: Vec<Note> =
            sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE space_id = ?1 AND is_deleted = FALSE")
                .bind(space_id)
                .fetch_all(&mut **transaction)
                .await?;

        // TODO: replace with `join_all`.
        for note in notes {
            SqliteDb::remove_note_inner(note.id, now, transaction).await?;
        }

        let space: Space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_deleted FROM spaces WHERE id = ?1 AND is_deleted = FALSE")
                .bind(space_id)
                .fetch_one(&mut **transaction)
                .await?;

        sqlx::query!(
            "UPDATE spaces SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2",
            now,
            space_id
        )
        .execute(&mut **transaction)
        .await?;

        sqlx::query!(
            "UPDATE files SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2",
            now,
            space.avatar_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn note_by_id(note_id: Uuid, connection: &mut SqliteConnection) -> Result<Note, DbError> {
        let note = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_deleted FROM notes WHERE id = ?1 AND is_deleted = FALSE")
            .bind(note_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(note)
    }

    pub async fn add_note(
        note: &Note,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at: _,
            updated_at: _,
            space_id,
            is_deleted: _,
        } = note;

        sqlx::query!(
            "INSERT INTO notes (id, text, created_at,  updated_at, space_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            text,
            now,
            now,
            space_id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn update_note(
        note: &Note,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at: _,
            updated_at: _,
            space_id: _,
            is_deleted,
        } = note;

        sqlx::query!(
            "UPDATE notes SET text = ?1, updated_at = ?2, is_deleted = ?3 WHERE id = ?4",
            text,
            now,
            is_deleted,
            id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn file_by_id(file_id: Uuid, connection: &mut SqliteConnection) -> Result<File, DbError> {
        let file = sqlx::query_as(
            "SELECT id, name, path, created_at, updated_at, is_deleted FROM files WHERE id = ?1 AND is_deleted = FALSE",
        )
        .bind(file_id)
        .fetch_one(&mut *connection)
        .await?;

        Ok(file)
    }

    pub async fn add_file(
        file: &File,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        let File {
            id,
            name,
            path,
            created_at: _,
            updated_at: _,
            is_deleted: _,
        } = file;

        sqlx::query!(
            "INSERT INTO files (id, name, path, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            id,
            name,
            path,
            now,
            now,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn remove_file(
        file_id: Uuid,
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        sqlx::query!(
            "UPDATE files SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2",
            now,
            file_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn set_note_files(
        note_id: Uuid,
        files: &[Uuid],
        now: OffsetDateTime,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DbError> {
        sqlx::query("UPDATE notes SET updated_at = ?1 WHERE id = ?2")
            .bind(now)
            .bind(note_id)
            .execute(&mut **transaction)
            .await?;

        sqlx::query("DELETE FROM notes_files WHERE note_id = ?1")
            .bind(note_id)
            .execute(&mut **transaction)
            .await?;

        for file_id in files {
            sqlx::query("INSERT INTO notes_files (note_id, file_id) VALUES (?1, ?2)")
                .bind(note_id)
                .bind(file_id)
                .execute(&mut **transaction)
                .await?;
        }

        Ok(())
    }
}

impl Db for SqliteDb {
    #[instrument(ret, skip(self))]
    async fn files(&self) -> Result<Vec<File>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as(
            "SELECT id, name, path, created_at, updated_at, is_deleted FROM files WHERE is_deleted = FALSE",
        )
        .fetch_all(&mut *connection)
        .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        SqliteDb::file_by_id(file_id, &mut connection).await
    }

    #[instrument(ret, skip(self))]
    async fn add_file(&self, file: &File) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateFile(Cow::Borrowed(file))).await?;
        let now = transaction.now();

        Self::add_file(file, now, transaction.transaction()).await?;
        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteFile(file_id)).await?;
        let now = transaction.now();

        SqliteDb::remove_file(file_id, now, transaction.transaction()).await?;
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
        SqliteDb::space_by_id(space_id, &mut connection).await
    }

    #[instrument(ret, skip(self))]
    async fn create_space(&self, space: &Space) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateSpace(Cow::Borrowed(space))).await?;
        let now = transaction.now();

        SqliteDb::add_space(space, now, transaction.transaction()).await?;
        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteSpace(space_id)).await?;
        let now = transaction.now();

        SqliteDb::remove_space(space_id, now, transaction.transaction()).await?;
        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn update_space(&self, space: &Space) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::UpdateSpace(Cow::Borrowed(space))).await?;
        let now = transaction.now();

        SqliteDb::update_space(space, now, transaction.transaction()).await?;
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

        SqliteDb::note_by_id(note_id, &mut connection).await
    }

    #[instrument(ret, skip(self))]
    async fn create_note(&self, note: &Note) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateNote(Cow::Borrowed(note))).await?;
        let now = transaction.now();

        SqliteDb::add_note(note, now, transaction.transaction()).await?;
        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteNote(note_id)).await?;

        SqliteDb::remove_note_inner(note_id, transaction.now(), transaction.transaction()).await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::UpdateNote(Cow::Borrowed(note))).await?;
        let now = transaction.now();

        SqliteDb::update_note(note, now, transaction.transaction()).await?;
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
        let now = transaction.now();

        SqliteDb::set_note_files(note_id, files, now, transaction.transaction()).await?;
        transaction.commit().await?;

        Ok(())
    }
}
