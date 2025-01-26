use serde::{Deserialize, Serialize};

use crate::{BlockId, NoteChecksumValue, NoteData, NoteId, SpaceChecksumValue, SpaceData, SpaceId, UserId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub id: SpaceId,
    pub data: SpaceData,
    pub checksum: SpaceChecksumValue,
    pub user_id: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: NoteId,
    pub data: NoteData,
    pub checksum: NoteChecksumValue,
    pub space_id: SpaceId,
    pub block_id: BlockId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteIds {
    pub ids: Vec<NoteId>,
}
