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
        let files = sqlx::query_as("SELECT id, name, path, is_synced FROM files")
            .fetch_all(&self.pool)
            .await?;

        Ok(files)
    }

    #[instrument(ret, skip(self))]
    async fn file_by_id(&self, file_id: Uuid) -> Result<File, DbError> {
        let files = sqlx::query_as("SELECT id, name, path, is_synced FROM files WHERE id=?1")
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
            is_synced,
        } = file;

        sqlx::query!(
            "INSERT INTO files (id, name, path, is_synced) VALUES (?1, ?2, ?3, ?4)",
            id,
            name,
            path,
            is_synced
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
        let spaces = sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_synced FROM spaces")
            .fetch_all(&self.pool)
            .await?;

        Ok(spaces)
    }

    #[instrument(ret, skip(self))]
    async fn space_by_id(&self, space_id: Uuid) -> Result<Space, DbError> {
        let space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_synced FROM spaces WHERE id = ?1")
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
            is_synced,
        } = space;

        sqlx::query!(
            "INSERT INTO spaces (id, name, avatar_id, created_at, updated_at, is_synced) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            is_synced,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        let notes: Vec<Note> =
            sqlx::query_as("SELECT id, text, created_at, updated_at, space_id FROM notes WHERE space_id = ?1")
                .bind(space_id)
                .fetch_all(&mut *transaction)
                .await?;

        // TODO: replace manual deleting with `ON CASCADE` constraint.
        for note in notes {
            SqliteDb::remove_note_inner(note.id, &mut transaction).await?;
        }

        let space: Space =
            sqlx::query_as("SELECT id, name, avatar_id, created_at, updated_at, is_synced FROM spaces WHERE id = ?1")
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
            is_synced,
        } = space;

        sqlx::query!(
            "UPDATE spaces SET name = ?1, avatar_id = ?2, is_synced = ?3, updated_at = ?4 WHERE id = ?5",
            name,
            avatar_id,
            is_synced,
            updated_at,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn notes(&self) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_synced FROM notes")
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as(
            "SELECT id, text, created_at, updated_at, space_id, is_synced FROM notes WHERE space_id = ?1",
        )
        .bind(space_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(notes)
    }

    #[instrument(ret, skip(self))]
    async fn note_by_id(&self, note_id: Uuid) -> Result<Note, DbError> {
        let note =
            sqlx::query_as("SELECT id, text, created_at, updated_at, space_id, is_synced FROM notes WHERE id = ?1")
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
            is_synced,
        } = note;

        sqlx::query!(
            "INSERT INTO notes (id, text, created_at,  updated_at, space_id, is_synced) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            id,
            text,
            created_at,
            updated_at,
            space_id,
            is_synced,
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
            is_synced,
        } = note;

        sqlx::query!(
            "UPDATE notes SET text = ?1, is_synced = ?2, updated_at = ?3 WHERE id = ?4",
            text,
            is_synced,
            updated_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn note_files(&self, note_id: Uuid) -> Result<Vec<File>, DbError> {
        let files = sqlx::query_as(
            "SELECT files.id, files.name, files.path, files.is_synced
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
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;

    #[sqlx::test]
    async fn space_crud(pool: SqlitePool) {
        let db = SqliteDb::new(pool);

        let file_id = Uuid::new_v4();
        let file = File {
            id: file_id,
            name: "cat.jpg".into(),
            path: "/home/tbt/cat-01.jpg".into(),
            is_synced: false,
        };

        db.add_file(&file).await.unwrap();

        let new_avatar_id = Uuid::new_v4();
        let new_avatar = File {
            id: new_avatar_id,
            name: "cat-2.jpg".into(),
            path: "/home/tbt/cat-02.jpg".into(),
            is_synced: false,
        };

        db.add_file(&new_avatar).await.unwrap();

        //------

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let space = Space {
            id,
            name: "Tbt".into(),
            avatar_id: file_id,
            created_at,
            updated_at,
            is_synced: false,
        };

        db.create_space(&space).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert_eq!(Some(space), spaces.into_iter().find(|space| space.id == id));

        let updated_space = Space {
            id,
            name: "TheBestTvarynka".into(),
            avatar_id: new_avatar_id,
            created_at,
            updated_at,
            is_synced: false,
        };
        db.update_space(&updated_space).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert_eq!(Some(updated_space), spaces.into_iter().find(|space| space.id == id));

        db.remove_space(id).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert!(!spaces.iter().any(|space| space.id == id));

        //------

        db.remove_file(file_id).await.unwrap();
        db.remove_file(new_avatar_id).await.unwrap();
    }

    #[sqlx::test]
    async fn file_crud(pool: SqlitePool) {
        let db = SqliteDb::new(pool);

        let id = Uuid::new_v4();
        let file = File {
            id,
            name: "cat.jpg".into(),
            path: "/home/tbt/cat-01.jpg".into(),
            is_synced: false,
        };

        db.add_file(&file).await.unwrap();

        let files = db.files().await.unwrap();
        assert_eq!(Some(file), files.into_iter().find(|file| file.id == id));

        let updated_file = File {
            id,
            name: "cat-2.jpg".into(),
            path: "/home/tbt/cat-02.jpg".into(),
            is_synced: false,
        };
        db.update_file(&updated_file).await.unwrap();

        let files = db.files().await.unwrap();
        assert_eq!(Some(updated_file), files.into_iter().find(|file| file.id == id));

        db.remove_file(id).await.unwrap();

        let files = db.files().await.unwrap();
        assert!(!files.iter().any(|file| file.id == id));
    }

    #[sqlx::test]
    async fn note_crud(pool: SqlitePool) {
        let db = SqliteDb::new(pool);

        let file_id = Uuid::new_v4();
        let file = File {
            id: file_id,
            name: "cat.jpg".into(),
            path: "/home/tbt/cat-01.jpg".into(),
            is_synced: false,
        };

        db.add_file(&file).await.unwrap();

        let space_id = Uuid::new_v4();
        let space = Space {
            id: space_id,
            name: "Test Notes CRUD".into(),
            avatar_id: file_id,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            is_synced: false,
        };

        db.create_space(&space).await.unwrap();

        //------

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let note = Note {
            id,
            text: "some text 1".into(),
            space_id,
            created_at,
            updated_at,
            is_synced: false,
        };

        db.create_note(&note).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert_eq!(Some(note), notes.into_iter().find(|note| note.id == id));

        let updated_note = Note {
            id,
            text: "some text 2".into(),
            space_id,
            created_at,
            updated_at,
            is_synced: false,
        };
        db.update_note(&updated_note).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert_eq!(Some(updated_note), notes.into_iter().find(|note| note.id == id));

        db.remove_note(id).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert!(!notes.iter().any(|note| note.id == id));

        //------

        db.remove_space(space_id).await.unwrap();
        db.remove_file(file_id).await.unwrap();
    }
}
