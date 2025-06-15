mod operation;

pub use operation::{Operation, OperationLogger, OperationOwned, OperationRecord, OperationRecordOwned};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::sync::{Hash, Hasher};

#[derive(Debug, FromRow, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Space {
    pub id: Uuid,
    pub name: String,
    pub avatar_id: Uuid,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    pub is_deleted: bool,
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
            is_deleted: false,
        }
    }
}

impl Hash for Space {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.avatar_id.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
        self.is_deleted.hash(state);
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    pub space_id: Uuid,
    pub is_deleted: bool,
}

impl Hash for Note {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.text.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
        self.space_id.hash(state);
        self.is_deleted.hash(state);
    }
}

impl Note {
    pub fn new(id: Uuid, text: String, created_at: OffsetDateTime, updated_at: OffsetDateTime, space_id: Uuid) -> Self {
        Self {
            id,
            text,
            created_at,
            updated_at,
            space_id,
            is_deleted: false,
        }
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    pub is_deleted: bool,
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.path.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
        self.is_deleted.hash(state);
    }
}

impl File {
    pub fn new(id: Uuid, name: String, path: String, created_at: OffsetDateTime, updated_at: OffsetDateTime) -> Self {
        Self {
            id,
            name,
            path,
            created_at,
            updated_at,
            is_deleted: false,
        }
    }
}
