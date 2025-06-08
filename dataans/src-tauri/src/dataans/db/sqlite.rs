use std::borrow::Cow;

use sqlx::{SqlitePool, Transaction};
use uuid::Uuid;

use super::*;

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
    // TODO: remove this function and add `ON CASCADE` sql constraint instead.
    #[instrument(ret, skip(transaction))]
    async fn remove_note_inner(note_id: Uuid, transaction: &mut Transaction<'_, sqlx::Sqlite>) -> Result<(), DbError> {
        let note_files: Vec<File> = sqlx::query_as(
            "SELECT files.id, files.name, files.path
            FROM files
                LEFT JOIN notes_files ON files.id = notes_files.file_id
            WHERE notes_files.note_id = ?1",
        )
        .bind(note_id)
        .fetch_all(&mut **transaction)
        .await?;

        sqlx::query("DELETE FROM notes_files WHERE note_id = ?1")
            .bind(note_id)
            .execute(&mut **transaction)
            .await?;

        for file in note_files {
            sqlx::query!("DELETE FROM files WHERE id = ?1", file.id)
                .execute(&mut **transaction)
                .await?;
        }

        sqlx::query!("DELETE FROM notes WHERE id = ?1", note_id)
            .execute(&mut **transaction)
            .await?;

        Ok(())
    }
}

impl Db for SqliteDb {
    #[instrument(ret, skip(self))]
    async fn files(&self) -> Result<Vec<File>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as("SELECT id, name, path FROM files")
            .fetch_all(&mut *connection)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let files = sqlx::query_as("SELECT id, name, path FROM files WHERE id=?1")
            .bind(file_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn add_file(&self, file: &File) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::CreateFile(Cow::Borrowed(file))).await?;

        let File { id, name, path } = file;

        sqlx::query!("INSERT INTO files (id, name, path) VALUES (?1, ?2, ?3)", id, name, path,)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin(Operation::DeleteFile(file_id)).await?;

        sqlx::query!("DELETE FROM files WHERE id = ?1", file_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn spaces(&self) -> Result<Vec<Space>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let spaces = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at FROM spaces")
            .fetch_all(&mut *connection)
            .await?;

        Ok(spaces)
    }

    #[instrument(ret, skip(self))]
    async fn space_by_id(&self, space_id: Uuid) -> Result<Space, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let space = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at FROM spaces WHERE id = ?1")
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
            sqlx::query_as("SELECT id, text, created_at, updated_at, space_id FROM notes WHERE space_id = ?1")
                .bind(space_id)
                .fetch_all(&mut *transaction)
                .await?;

        // TODO: replace manual deleting with `ON CASCADE` constraint.
        for note in notes {
            SqliteDb::remove_note_inner(note.id, transaction.transaction()).await?;
        }

        let space: Space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at FROM spaces WHERE id = ?1")
                .bind(space_id)
                .fetch_one(&mut *transaction)
                .await?;

        sqlx::query!("DELETE FROM spaces WHERE id = ?1", space_id)
            .execute(&mut *transaction)
            .await?;

        sqlx::query!("DELETE FROM files WHERE id = ?1", space.avatar_id)
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

        let notes = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id FROM notes")
            .fetch_all(&mut *connection)
            .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let notes = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id FROM notes WHERE space_id = ?1")
            .bind(space_id)
            .fetch_all(&mut *connection)
            .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn note_by_id(&self, note_id: Uuid) -> Result<Note, DbError> {
        let mut connection = self.pool.read_only_connection().await?;

        let note = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id FROM notes WHERE id = ?1")
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

        let files = sqlx::query_as(
            "SELECT files.id, files.name, files.path
            FROM files
                LEFT JOIN notes_files ON files.id = notes_files.file_id
            WHERE notes_files.note_id = ?1",
        )
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
