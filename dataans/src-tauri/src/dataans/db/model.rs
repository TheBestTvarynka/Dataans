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
}

impl Space {
    pub fn new(
        id: Uuid,
        name: String,
        avatar_id: Uuid,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub space_id: Uuid,
}

impl Note {
    pub fn new(id: Uuid, text: String, created_at: OffsetDateTime, updated_at: OffsetDateTime, space_id: Uuid) -> Self {
        Self {
            id,
            text,
            created_at,
            updated_at,
            space_id,
        }
    }
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
}

impl File {
    pub fn new(id: Uuid, name: String, path: String) -> Self {
        Self { id, name, path }
    }
}
