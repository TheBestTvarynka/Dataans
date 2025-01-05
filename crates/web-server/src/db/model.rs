use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::PrimitiveDateTime;
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
    pub created_at: PrimitiveDateTime,
    pub expiration_date: PrimitiveDateTime,
}
