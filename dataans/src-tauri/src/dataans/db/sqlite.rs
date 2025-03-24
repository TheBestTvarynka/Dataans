use sqlx::{SqlitePool, Transaction};
use uuid::Uuid;

use super::*;

pub struct SqliteDb {
    pool: SqlitePool,
}

impl SqliteDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl SqliteDb {
    // TODO: remove this function and add `ON CASCADE` sql constraint instead.
    #[instrument(ret, skip(transaction))]
    async fn remove_note_inner(note_id: Uuid, transaction: &mut Transaction<'_, sqlx::Sqlite>) -> Result<(), DbError> {
        let note_files: Vec<File> = sqlx::query_as(
            "SELECT files.id, files.name, files.path, files.checksum
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
        let files = sqlx::query_as("SELECT id, name, path, checksum FROM files")
            .fetch_all(&self.pool)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let files = sqlx::query_as("SELECT id, name, path, checksum FROM files WHERE id=?1")
            .bind(file_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn add_file(&self, file: &File) -> Result<(), DbError> {
        let File {
            id,
            name,
            path,
            checksum,
        } = file;

        sqlx::query!(
            "INSERT INTO files (id, name, path, checksum) VALUES (?1, ?2, ?3, ?4)",
            id,
            name,
            path,
            checksum
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM files WHERE id = ?1", file_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn spaces(&self) -> Result<Vec<Space>, DbError> {
        let spaces = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, checksum FROM spaces")
            .fetch_all(&self.pool)
            .await?;

        Ok(spaces)
    }

    #[instrument(ret, skip(self))]
    async fn space_by_id(&self, space_id: Uuid) -> Result<Space, DbError> {
        let space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, checksum FROM spaces WHERE id = ?1")
                .bind(space_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(space)
    }

    #[instrument(ret, skip(self))]
    async fn create_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            checksum,
        } = space;

        sqlx::query!(
            "INSERT INTO spaces (id, name, avatar_id, created_at, updated_at, checksum) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            checksum,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        let notes: Vec<Note> = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, checksum, block_id FROM notes WHERE space_id = ?1",
        )
        .bind(space_id)
        .fetch_all(&mut *transaction)
        .await?;

        // TODO: replace manual deleting with `ON CASCADE` constraint.
        for note in notes {
            SqliteDb::remove_note_inner(note.id, &mut transaction).await?;
        }

        let space: Space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, checksum FROM spaces WHERE id = ?1")
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
        let Space {
            id,
            name,
            avatar_id,
            created_at: _,
            updated_at,
            checksum,
        } = space;

        sqlx::query!(
            "UPDATE spaces SET name = ?1, avatar_id = ?2, checksum = ?3, updated_at = ?4 WHERE id = ?5",
            name,
            avatar_id,
            checksum,
            updated_at,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn notes(&self) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, checksum, block_id FROM notes")
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, checksum, block_id FROM notes WHERE space_id = ?1",
        )
        .bind(space_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn note_by_id(&self, note_id: Uuid) -> Result<Note, DbError> {
        let note = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, checksum, block_id FROM notes WHERE id = ?1",
        )
        .bind(note_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(note)
    }

    #[instrument(ret, skip(self))]
    async fn create_note(&self, note: &Note) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at,
            updated_at,
            space_id,
            checksum,
            block_id: _,
        } = note;

        sqlx::query!(
            "INSERT INTO notes (id, text, created_at,  updated_at, space_id, checksum) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            id,
            text,
            created_at,
            updated_at,
            space_id,
            checksum,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        SqliteDb::remove_note_inner(note_id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at: _,
            updated_at,
            space_id: _,
            checksum,
            block_id,
        } = note;

        sqlx::query!(
            "UPDATE notes SET text = ?1, checksum = ?2, updated_at = ?3, block_id = ?4 WHERE id = ?5",
            text,
            checksum,
            updated_at,
            block_id,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn note_files(&self, note_id: Uuid) -> Result<Vec<File>, DbError> {
        let files = sqlx::query_as(
            "SELECT files.id, files.name, files.path, files.checksum
            FROM files
                LEFT JOIN notes_files ON files.id = notes_files.file_id
            WHERE notes_files.note_id = ?1",
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn set_note_files(&self, note_id: Uuid, files: &[Uuid]) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

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

    #[instrument(ret, skip(self))]
    async fn blocks(&self) -> Result<Vec<SyncBlock>, DbError> {
        let blocks = sqlx::query_as("SELECT id, checksum, space_id FROM sync_blocks")
            .fetch_all(&self.pool)
            .await?;

        Ok(blocks)
    }

    async fn block_notes(&self, block_id: Uuid) -> Result<Vec<SyncBlockNote>, DbError> {
        let notes = sqlx::query_as("SELECT id, checksum, block_id FROM notes WHERE block_id = ?1")
            .bind(block_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn unsynced_notes(&self) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, checksum, block_id FROM notes WHERE block_id IS NULL",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(notes)
    }
}
