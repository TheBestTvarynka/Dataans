use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

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

impl PostgresDb {
    async fn operation_by_id(&self, operation_id: Uuid, connection: &mut PgConnection) -> Result<Operation, DbError> {
        let operation = sqlx::query_as("select id, created_at, data, checksum from operation where id = $1")
            .bind(operation_id)
            .fetch_one(&mut *connection)
            .await?;

        Ok(operation)
    }
}

impl OperationsDb for PostgresDb {
    async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, DbError> {
        let operations =
            sqlx::query_as("select id, created_at, data, checksum from operation order by created_at offset $1")
                .bind(i64::try_from(operations_to_skip).expect("usize -> i64 conversion should not fail"))
                .fetch_all(&self.pool)
                .await?;

        Ok(operations)
    }

    async fn add_operations(&self, operations: &[Operation]) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        // TODO: replace with `join_all`.
        for operation in operations {
            if let Ok(existing_operation) = self.operation_by_id(operation.id, &mut transaction).await {
                warn!(
                    "Operation ({}) reuploading detected. It is allowed but unwanted behaviour!",
                    operation.id
                );

                if existing_operation.checksum != operation.checksum {
                    error!(
                        "Operation with id ({}) always exists but provided operation checksum ({:?}) do not match existing operation checksum ({:?}).",
                        operation.id, operation.checksum, existing_operation.checksum
                    );

                    return Err(DbError::InvalidOperation(
                        "operation with the same id but different checksum already exists",
                    ));
                }
            } else {
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
        }

        transaction.commit().await?;

        Ok(())
    }
}
