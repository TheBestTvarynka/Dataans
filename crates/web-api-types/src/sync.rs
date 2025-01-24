use derive_more::{AsRef, From};
use serde::{Deserialize, Serialize};

use crate::{BlockId, NoteId, SpaceId};

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct NoteChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct BlockChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct BlockNumber(i32);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteChecksum {
    pub id: NoteId,
    pub checksum: NoteChecksumValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockNotes {
    pub block_id: BlockId,
    pub notes: Vec<NoteChecksum>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncBlock {
    pub id: BlockId,
    pub number: BlockNumber,
    pub checksum: BlockChecksumValue,
    pub space_id: SpaceId,
}
