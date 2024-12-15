use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Space {
    pub id: Uuid,
    pub name: String,
    pub avatar_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: OffsetDateTime,
    pub space_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
}
