use derive_more::{AsRef, From};
use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::{Duration, OffsetDateTime};
use url::Url;
use web_api_types::{AuthToken, UserId, Username};

/// Secret key.
///
/// This key is used to encrypt the user's data.
#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct SecretKey(Vec<u8>);

/// Web server URL.
///
/// The authentication and synchronization server URL.
#[derive(Debug, Serialize, Deserialize, AsRef, From, Clone)]
pub struct WebServerUrl(Url);

/// Synchronization mode.
///
/// It represents how the user wants to synchronize the data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SyncMode {
    /// The user manually synchronizes the data by pressing the sync button.
    Manual,
    /// The app maintains the websocket connection with the server and automatically synchronize the data.
    Push,
    /// The app periodically polls the server to check if there are any changes.
    Poll {
        /// The polling interval. Basically, how often the app should check for changes.
        delay: Duration,
    },
}

/// Synchronization configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sync {
    /// No synchronization enabled.
    ///
    /// The user needs to sign up/in to enable synchronization.
    Disabled {
        /// The synchronization server URL.
        url: WebServerUrl,
    },
    /// The user is authorized but synchronization is not enabled.
    /// The synchronization is enabled.
    Enabled {
        /// The synchronization server URL.
        url: WebServerUrl,
        /// The synchronization mode. It represents how the user wants to synchronize the data.
        mode: SyncMode,
    },
}

/// User profile.
///
/// Represents the user's profile.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID.
    pub user_id: UserId,
    /// Username.
    pub username: Username,
    /// Auth token.
    pub auth_token: AuthToken,
    /// Auth token expiration date.
    #[serde(with = "rfc3339")]
    pub auth_token_expiration_date: OffsetDateTime,
    /// Secret key.
    pub secret_key: SecretKey,
    /// Synchronization configuration.
    pub sync_config: Sync,
}

/// User context.
///
/// The user context is returned by the backend and used only on frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    /// User ID.
    pub user_id: UserId,
    /// Username.
    pub username: Username,
    /// Synchronization configuration.
    pub sync_config: Sync,
}
