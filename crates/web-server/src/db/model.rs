use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Operation {
    pub id: Uuid,
    pub created_at: OffsetDateTime,
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub secret_key_hash: String,
}
