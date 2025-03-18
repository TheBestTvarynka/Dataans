use derive_more::{AsRef, From};
use serde::{Deserialize, Serialize};

use crate::{BlockChecksumValue, BlockId, NoteChecksumValue, NoteId, SpaceId};

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
    // TODO: remove block number.
    pub number: BlockNumber,
    pub checksum: BlockChecksumValue,
    pub space_id: SpaceId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockIds {
    pub ids: Vec<BlockId>,
}
