use derive_more::{AsRef, From, Into};
use serde::{Deserialize, Serialize};
use url::Url;

/// Authorization token.
///
/// This token is used to authenticate the user with the backend server.
#[derive(Debug, Serialize, Deserialize, AsRef, From, Into, Clone)]
pub struct AuthorizationToken(String);

/// Key derivation salt (nonce).
///
/// The salt is used to derive the encryption key from the user's password.
#[derive(Debug, Serialize, Deserialize, AsRef, From, Into, Clone)]
pub struct Salt(String);

/// Secret key.
///
/// This key is used to encrypt the user's data.
#[derive(Debug, Serialize, Deserialize, AsRef, From, Into, Clone)]
pub struct SecretKey(Vec<u8>);

/// Web server URL.
///
/// The authentication and synchronization server URL.
#[derive(Debug, Serialize, Deserialize, AsRef, From, Into, Clone)]
pub struct WebServerUrl(Url);

/// Synchronization mode.
///
/// It represents how the user wants to synchronize the data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncMode {
    /// The user manually synchronizes the data by pressing the sync button.
    Manual,
    // /// The app maintains the websocket connection with the server and automatically synchronize the data.
    // Push,
}

/// Synchronization configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sync {
    /// The synchronization server URL.
    pub url: WebServerUrl,
    /// The synchronization mode. It represents how the user wants to synchronize the data.
    pub mode: SyncMode,
}

/// User profile.
///
/// Represents the user's profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    /// Authorization token.
    pub auth_token: AuthorizationToken,
    /// Secret key.
    pub secret_key: SecretKey,
    /// Key derivation salt (nonce).
    ///
    /// The app does not need this value. It is stored only for the user to be able read it
    /// and login on another device.
    pub salt: Salt,
    /// Synchronization configuration.
    pub sync_config: Sync,
}

/// User context.
///
/// The user context is returned by the backend and used only on frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    /// Synchronization configuration.
    pub sync_config: Sync,
}
