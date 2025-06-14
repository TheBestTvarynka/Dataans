use derive_more::{Into, From, AsRef};
use serde::{Deserialize, Serialize};

use crate::{OperationId, OperationData, OperationChecksumValue, CreationDate};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub id: OperationId,
    pub created_at: CreationDate,
    pub data: OperationData,
    pub checksum: OperationChecksumValue,
}

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct BlockChecksum(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct Blocks(Vec<BlockChecksum>);
