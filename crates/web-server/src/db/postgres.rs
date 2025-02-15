use sha2::{Digest, Sha256};
use sqlx::{PgPool, Transaction};
use uuid::Uuid;

use super::model::*;
use super::{AuthDb, DbError, NoteDb, SpaceDb, SyncDb};
use crate::crypto::{Sha256Checksum, EMPTY_SHA256_CHECKSUM};

const MAX_NOTES_IN_BLOCK: i64 = 32;

pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn update_block_checksum(
        &self,
        block_id: Uuid,
        transaction: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), DbError> {
        let note_checksums: Vec<(Vec<u8>,)> =
            sqlx::query_as("select checksum from note where block_id = $1 order by id desc")
                .bind(block_id)
                .fetch_all(&mut **transaction)
                .await?;

        let mut hasher = Sha256::new();

        note_checksums
            .into_iter()
            .for_each(|note_checksum| hasher.update(note_checksum.0));

        let block_checksum: Sha256Checksum = hasher.finalize().into();

        sqlx::query!(
            "update sync_block set checksum = $1 where id = $2",
            block_checksum.as_slice(),
            block_id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
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

    async fn session(&self, session_id: Uuid) -> Result<Session, DbError> {
        let session = sqlx::query_as("select id, user_id, created_at, expiration_date from session where id = $1")
            .bind(session_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(session)
    }

    async fn remove_user(&self, user_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!("delete from \"user\" where id = $1", user_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl SpaceDb for PostgresDb {
    async fn space(&self, space_id: Uuid) -> Result<Space, DbError> {
        let space = sqlx::query_as("select id, data, checksum, user_id from space where id = $1")
            .bind(space_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(space)
    }

    async fn add_space(&self, space: &Space) -> Result<(), DbError> {
        let Space {
            id: space_id,
            data,
            checksum,
            user_id,
        } = space;

        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            "insert into space (id, data, checksum, user_id) values ($1, $2, $3, $4)",
            space_id,
            data,
            checksum,
            user_id,
        )
        .execute(&mut *transaction)
        .await?;

        // Create the first sync block for a new space
        sqlx::query!(
            "insert into sync_block (id, number, checksum, space_id) values (gen_random_uuid(), 1, $1, $2)",
            EMPTY_SHA256_CHECKSUM,
            space_id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

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

    async fn remove_space(&self, space_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!("delete from space where id = $1", space_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl NoteDb for PostgresDb {
    async fn notes(&self, note_ids: &[Uuid]) -> Result<Vec<Note>, DbError> {
        let notes = sqlx::query_as("select id, data, checksum, space_id, block_id from note where id = any($1)")
            .bind(note_ids)
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn add_note(&self, note: &Note) -> Result<Uuid, DbError> {
        let Note {
            id: node_id,
            data,
            checksum,
            space_id,
            block_id: note_block_id,
        } = note;

        let mut transaction = self.pool.begin().await?;

        let (notes_in_block, block_id, block_number): (i64, Uuid, i32) = sqlx::query_as(
            "select count(note.id), sync_block.id, sync_block.number
                from sync_block left join note on note.block_id = sync_block.id
                where sync_block.space_id = $1
                group by sync_block.id order by number desc limit 1",
        )
        .bind(space_id)
        .fetch_one(&mut *transaction)
        .await?;

        trace!(notes_in_block, ?block_id, block_number);

        let block_id = if notes_in_block >= MAX_NOTES_IN_BLOCK {
            trace!("Creating new block");

            let block_number = block_number + 1;

            sqlx::query!(
                "insert into sync_block (id, number, checksum, space_id) values ($1, $2, $3, $4)",
                note_block_id,
                block_number,
                // We are going to update the checksum later anyway
                EMPTY_SHA256_CHECKSUM,
                space_id,
            )
            .execute(&mut *transaction)
            .await?;

            *note_block_id
        } else {
            trace!("Adding note to existing block");

            block_id
        };

        sqlx::query!(
            "insert into note (id, data, checksum, space_id, block_id) values ($1, $2, $3, $4, $5)",
            node_id,
            data,
            checksum,
            space_id,
            block_id,
        )
        .execute(&mut *transaction)
        .await?;

        self.update_block_checksum(block_id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(block_id)
    }

    async fn update_note(&self, note: &Note) -> Result<(), DbError> {
        let Note {
            id: node_id,
            data,
            checksum,
            space_id: _,
            block_id,
        } = note;

        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            "update note set data = $1, checksum = $2 where id = $3",
            data,
            checksum,
            node_id,
        )
        .execute(&mut *transaction)
        .await?;


        self.update_block_checksum(*block_id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn remove_note(&self, note_id: Uuid) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        let (block_id,): (Uuid,) = sqlx::query_as("select block_id from note where id = $1")
            .bind(note_id)
            .fetch_one(&mut *transaction)
            .await?;

        sqlx::query!("delete from note where id = $1", note_id)
            .execute(&mut *transaction)
            .await?;

        self.update_block_checksum(block_id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn note_owner(&self, note_id: Uuid) -> Result<Uuid, DbError> {
        let (owner_id,): (Uuid,) =
            sqlx::query_as("select user_id from note left join space on note.space_id = space.id where note.id = $1")
                .bind(note_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(owner_id)
    }
}

impl SyncDb for PostgresDb {
    async fn blocks(&self, space_id: Uuid) -> Result<Vec<SyncBlock>, DbError> {
        let blocks = sqlx::query_as("select id, number, checksum, space_id from sync_block where space_id = $1")
            .bind(space_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(blocks)
    }

    async fn block_notes(&self, block_id: Uuid) -> Result<Vec<NoteChecksum>, DbError> {
        let notes = sqlx::query_as("select id, checksum from note where block_id = $1")
            .bind(block_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn block_owner(&self, block_id: Uuid) -> Result<Uuid, DbError> {
        let (user_id,): (Uuid,) = sqlx::query_as("select space.user_id from sync_block join space on sync_block.space_id = space.id where sync_block.id = $1")
            .bind(block_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user_id)
    }
}
