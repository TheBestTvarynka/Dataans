use sqlx::SqlitePool;
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

impl Db for SqliteDb {
    async fn files(&self) -> Result<Vec<File>, DbError> {
        let files = sqlx::query_as("SELECT id, name, path FROM files")
            .fetch_all(&self.pool)
            .await?;

        Ok(files)
    }

    async fn add_file(&self, file: &File) -> Result<(), DbError> {
        let File { id, name, path } = file;

        sqlx::query!("INSERT INTO files (id, name, path) VALUES (?1, ?2, ?3)", id, name, path)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove_file(&self, file_id: Uuid) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM files WHERE id = ?1", file_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_file(&self, file: &File) -> Result<(), DbError> {
        let File { id, name, path } = file;

        sqlx::query!("UPDATE files SET name = ?1, path = ?2 WHERE id = ?3", name, path, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn spaces(&self) -> Result<Vec<Space>, DbError> {
        let spaces = sqlx::query_as("SELECT id, name, avatar_id, created_at FROM spaces")
            .fetch_all(&self.pool)
            .await?;

        Ok(spaces)
    }

    async fn create_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id,
            name,
            avatar_id,
            created_at,
        } = space;

        sqlx::query!(
            "INSERT INTO spaces (id, name, avatar_id, created_at) VALUES (?1, ?2, ?3, ?4)",
            id,
            name,
            avatar_id,
            created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM spaces WHERE id = ?1", space_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id,
            name,
            avatar_id,
            created_at: _,
        } = space;

        sqlx::query!(
            "UPDATE spaces SET name = ?1, avatar_id = ?2 WHERE id = ?3",
            name,
            avatar_id,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn space_notes(&self, space_id: Uuid) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as("SELECT id, text, created_at, space_id FROM notes WHERE space_id = ?1")
            .bind(space_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn create_note(&self, note: &Note) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at,
            space_id,
        } = note;

        sqlx::query!(
            "INSERT INTO notes (id, text, created_at, space_id) VALUES (?1, ?2, ?3, ?4)",
            id,
            text,
            created_at,
            space_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM notes WHERE id = ?1", note_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        let Note {
            id,
            text,
            created_at: _,
            space_id: _,
        } = note;

        sqlx::query!("UPDATE notes SET text = ?1 WHERE id = ?2", text, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;

    fn pool() -> SqlitePool {
        use crate::dataans::SqlitePoolOptions;

        SqlitePoolOptions::new()
            .max_connections(4)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect_lazy("sqlite:///home/pavlo-myroniuk/.local/share/com.tbt.dataans/db/dataans.sqlite")
            .expect("can not connect to sqlite db")
    }

    #[tokio::test]
    async fn space_crud() {
        let db = SqliteDb::new(pool());

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();
        let space = Space {
            id,
            name: "Tbt".into(),
            avatar_id: Uuid::from_str("620b74b0-05d7-4170-911f-6eeea7b15c44").unwrap(),
            created_at: created_at.clone(),
        };

        db.create_space(&space).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert_eq!(Some(space), spaces.into_iter().find(|space| space.id == id));

        let updated_space = Space {
            id,
            name: "TheBestTvarynka".into(),
            avatar_id: Uuid::from_str("620b74b0-05d7-4170-911f-6eeea7b15c44").unwrap(),
            created_at,
        };
        db.update_space(&updated_space).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert_eq!(Some(updated_space), spaces.into_iter().find(|space| space.id == id));

        db.remove_space(id).await.unwrap();

        let spaces = db.spaces().await.unwrap();
        assert!(spaces.iter().find(|space| space.id == id).is_none());
    }

    #[tokio::test]
    async fn file_crud() {
        let db = SqliteDb::new(pool());

        let id = Uuid::new_v4();
        let file = File {
            id,
            name: "cat.jpg".into(),
            path: "/home/tbt/cat-01.jpg".into(),
        };

        db.add_file(&file).await.unwrap();

        let files = db.files().await.unwrap();
        assert_eq!(Some(file), files.into_iter().find(|file| file.id == id));

        let updated_file = File {
            id,
            name: "cat-2.jpg".into(),
            path: "/home/tbt/cat-02.jpg".into(),
        };
        db.update_file(&updated_file).await.unwrap();

        let files = db.files().await.unwrap();
        assert_eq!(Some(updated_file), files.into_iter().find(|file| file.id == id));

        db.remove_file(id).await.unwrap();

        let files = db.files().await.unwrap();
        assert!(files.iter().find(|file| file.id == id).is_none());
    }

    #[tokio::test]
    async fn note_crud() {
        let db = SqliteDb::new(pool());

        let space_id = Uuid::new_v4();
        let space = Space {
            id: space_id,
            name: "Test Notes CRUD".into(),
            avatar_id: Uuid::from_str("620b74b0-05d7-4170-911f-6eeea7b15c44").unwrap(),
            created_at: OffsetDateTime::now_utc(),
        };

        db.create_space(&space).await.unwrap();

        //------

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();
        let note = Note {
            id,
            text: "some text 1".into(),
            space_id,
            created_at: created_at.clone(),
        };

        db.create_note(&note).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert_eq!(Some(note), notes.into_iter().find(|note| note.id == id));

        let updated_note = Note {
            id,
            text: "some text 2".into(),
            space_id,
            created_at,
        };
        db.update_note(&updated_note).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert_eq!(Some(updated_note), notes.into_iter().find(|note| note.id == id));

        db.remove_note(id).await.unwrap();

        let notes = db.space_notes(space_id).await.unwrap();
        assert!(notes.iter().find(|note| note.id == id).is_none());

        db.remove_space(space_id).await.unwrap();
    }
}
