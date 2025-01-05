use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
