use sqlx::SqlitePool;

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
    async fn space_creation() {
        let db = SqliteDb::new(pool());

        db.create_space(&Space {
            id: Uuid::new_v4(),
            name: "Tbt".into(),
            avatar_id: Uuid::from_str("620b74b0-05d7-4170-911f-6eeea7b15c44").unwrap(),
            created_at: OffsetDateTime::now_utc(),
        })
        .await
        .unwrap();

        let spaces = db.spaces().await.unwrap();
        println!("{:?}", spaces);
    }

    #[tokio::test]
    async fn file_creation() {
        let db = SqliteDb::new(pool());

        db.add_file(&File {
            id: Uuid::new_v4(),
            name: "cat.jpg".into(),
            path: "/home/tbt/cat-01.jpg".into(),
        })
        .await
        .unwrap();

        let files = db.files().await.unwrap();
        println!("{:?}", files);
    }
}
