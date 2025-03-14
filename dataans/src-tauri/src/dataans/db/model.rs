use sqlx::FromRow;
use sync_common::{Hash, Hasher, Sha256};
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

impl Space {
    pub fn new(id: Uuid, name: String, avatar_id: Uuid, created_at: OffsetDateTime, updated_at: OffsetDateTime) -> Self {
        let mut space = Self {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            checksum: Vec::new(),
        };

        space.checksum = space.digest::<Sha256>().to_vec();

        space
    }
}

impl Hash for Space {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.avatar_id.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
    }
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

impl Note {
    pub fn new(id: Uuid, text: String, created_at: OffsetDateTime, updated_at: OffsetDateTime, space_id: Uuid, block_id: Option<Uuid>) -> Self {
        let mut note = Self {
            id,
            text,
            created_at,
            updated_at,
            space_id,
            block_id,
            checksum: Vec::new(),
        };

        note.checksum = note.digest::<Sha256>().to_vec();

        note
    }
}

impl Hash for Note {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.text.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
    }
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub checksum: Vec<u8>,
}

impl File {
    pub fn new(id: Uuid, name: String, path: String) -> Self {
        let mut file = Self {
            id,
            name,
            path,
            checksum: Vec::new(),
        };

        file.checksum = file.digest::<Sha256>().to_vec();

        file
    }
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.path.hash(state);
    }
}