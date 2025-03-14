use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Space {
    pub id: Uuid,
    pub name: String,
    pub avatar_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub checksum: Vec<u8>,
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub space_id: Uuid,
    pub block_id: Option<Uuid>,
    pub checksum: Vec<u8>,
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub checksum: Vec<u8>,
}
