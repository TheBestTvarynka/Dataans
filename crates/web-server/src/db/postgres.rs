use sqlx::PgPool;

use super::{AuthDb, DbError};

pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AuthDb for PostgresDb {
    async fn create_user(&self) -> Result<(), DbError> {
        Ok(())
    }
}
