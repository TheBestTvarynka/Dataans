use derive_more::{AsRef, From, Into};
use serde::{Deserialize, Serialize};

use crate::{CreationDate, OperationChecksumValue, OperationData, OperationId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub id: OperationId,
    pub created_at: CreationDate,
    pub data: OperationData,
    pub checksum: OperationChecksumValue,
}

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct BlockChecksum(pub Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct Blocks(pub Vec<BlockChecksum>);
