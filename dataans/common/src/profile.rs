use nutype::nutype;
use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;
use web_api_types::{AuthToken, Username};

/// User ID.
#[nutype(derive(Debug, Serialize, Deserialize, AsRef, Deref, From))]
pub struct UserId(Uuid);

/// Secret key.
///
/// This key is used to encrypt the user's data.
#[nutype(derive(Debug, Serialize, Deserialize, AsRef, Deref, From))]
pub struct SecretKey(String);

/// User profile.
///
/// Represents the user's profile.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID.
    pub user_id: Uuid,
    /// Username.
    pub username: Username,
    /// Auth token.
    pub auth_token: AuthToken,
    /// Auth token expiration date.
    #[serde(with = "rfc3339")]
    pub auth_token_expiration_date: OffsetDateTime,
    /// Secret key.
    pub secret_key: SecretKey,
}
