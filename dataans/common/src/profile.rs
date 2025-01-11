use nutype::nutype;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_api_types::Username;

/// User ID.
#[nutype(derive(Debug, Serialize, Deserialize, AsRef, Deref, From))]
pub struct UserId(Uuid);

/// Auth token.
///
/// This token is used to authenticate the user. It is generated when the user logs in.
#[nutype(derive(Debug, Serialize, Deserialize, AsRef, Deref, From))]
pub struct AuthToken(String);

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
    /// Secret key.
    pub secret_key: SecretKey,
}
