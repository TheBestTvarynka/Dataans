use sqlx::PgPool;

use super::model::*;
use super::{DbError, OperationsDb};

pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl OperationsDb for PostgresDb {
    async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, DbError> {
        let operations =
            sqlx::query_as("select id, created_at, data, checksum from operation order by created_at offset $1")
                .bind(i64::try_from(operations_to_skip).expect("usize -> u32 conversion should not fail"))
                .fetch_all(&self.pool)
                .await?;

        Ok(operations)
    }

    async fn add_operations(&self, operations: &[Operation]) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        // TODO: replace with `join_all`.
        for operation in operations {
            let Operation {
                id,
                created_at,
                data,
                checksum,
            } = operation;

            sqlx::query!(
                "insert into operation (id, created_at, data, checksum) values ($1, $2, $3, $4)",
                id,
                created_at,
                data,
                checksum
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
