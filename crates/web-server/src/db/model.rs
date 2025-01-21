use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct InvitationToken {
    pub id: Uuid,
    pub data: Vec<u8>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: Vec<u8>,
    pub password: Vec<u8>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub expiration_date: OffsetDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Space {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SyncBlock {
    pub id: Uuid,
    pub number: i32,
    pub checksum: Vec<u8>,
    pub space_id: Uuid,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub space_id: Uuid,
    pub block_id: Uuid,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub note_id: Uuid,
}
