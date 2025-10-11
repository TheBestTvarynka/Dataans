use derive_more::{AsRef, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into)]
pub struct UserId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Clone, Into)]
pub struct SecretKeyHash(String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub secret_key_hash: SecretKeyHash,
}
