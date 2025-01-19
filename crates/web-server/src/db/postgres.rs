use sqlx::PgPool;
use uuid::Uuid;

use super::model::*;
use super::{AuthDb, DbError, NoteDb, SpaceDb};

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
    async fn find_invitation_token(&self, token: &[u8]) -> Result<InvitationToken, DbError> {
        let token = sqlx::query_as("select id, data from invitation_token where data=$1")
            .bind(token)
            .fetch_one(&self.pool)
            .await?;

        Ok(token)
    }

    #[instrument(ret, skip(self))]
    async fn add_user(&self, user: &User, token_id: Uuid) -> Result<(), DbError> {
        let User { id, username, password } = user;

        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            "insert into \"user\" (id, username, password) values ($1, $2, $3)",
            id,
            username,
            password
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "insert into used_invitation_token (token_id, user_id, used_at) values ($1, $2, now())",
            token_id,
            id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn find_user_by_username(&self, username: &[u8]) -> Result<User, DbError> {
        let user = sqlx::query_as("select id, username, password from \"user\" where username=$1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn add_session(&self, session: &Session) -> Result<(), DbError> {
        let Session {
            id,
            user_id,
            created_at,
            expiration_date,
        } = session;

        sqlx::query!(
            "insert into session (id, user_id, created_at, expiration_date) values ($1, $2, $3, $4)",
            id,
            user_id,
            created_at,
            expiration_date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl SpaceDb for PostgresDb {
    async fn add_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id,
            data,
            checksum,
            user_id,
        } = space;

        sqlx::query!(
            "insert into space (id, data, checksum, user_id) values ($1, $2, $3, $4)",
            id,
            data,
            checksum,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id,
            data,
            checksum,
            user_id: _,
        } = space;

        sqlx::query!(
            "update space set data = $1, checksum = $2 where id = $3",
            data,
            checksum,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_space(&self, _space_id: Uuid) -> Result<(), DbError> {
        Err(DbError::Unsupported("space deletion"))
    }
}

impl NoteDb for PostgresDb {
    async fn add_note(&self, note: &Note) -> Result<(), DbError> {
        Ok(())
    }

    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        Ok(())
    }

    async fn delete_note(&self, note_id: Uuid) -> Result<(), DbError> {
        Err(DbError::Unsupported("node deletion"))
    }
}
