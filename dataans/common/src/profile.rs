use derive_more::{AsRef, From, Into};
use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::OffsetDateTime;
use url::Url;
use web_api_types::{AuthToken, UserId, Username};

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
    /// The app maintains the websocket connection with the server and automatically synchronize the data.
    Push,
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

impl Sync {
    /// Returns web server url.
    pub fn get_web_server_url(&self) -> WebServerUrl {
        match self {
            Sync::Disabled { url } => url.clone(),
            Sync::Enabled { url, mode: _ } => url.clone(),
        }
    }

    /// Checks synchronization is enabled.
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Sync::Disabled { .. })
    }

    /// Returns [SyncMode] if the sync is enabled.
    pub fn mode(&self) -> Option<SyncMode> {
        if let Sync::Enabled { url: _, mode } = self {
            Some(*mode)
        } else {
            None
        }
    }
}

/// User profile.
///
/// Represents the user's profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
