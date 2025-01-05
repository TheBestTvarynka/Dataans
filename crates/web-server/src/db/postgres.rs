use sqlx::PgPool;
use uuid::Uuid;

use super::model::*;
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
    #[instrument(ret, skip(self))]
    async fn find_invitation_token(&self, token: &[u8]) -> Result<Option<InvitationToken>, DbError> {
        let token = sqlx::query_as("select id, data from invitation_token where data=$1")
            .bind(token)
            .fetch_one(&self.pool)
            .await?;

        Ok(Some(token))
    }

    #[instrument(ret, skip(self))]
    async fn add_user(&self, user: &User) -> Result<(), DbError> {
        let User { id, username, password } = user;

        sqlx::query!(
            "insert into \"user\" (id, username, password) values ($1, $2, $3)",
            id,
            username,
            password
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(ret, skip(self))]
    async fn assign_invitation_token(&self, token_id: Uuid, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "insert into used_invitation_token (token_id, user_id, used_at) values ($1, $2, now())",
            token_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
