use std::path::PathBuf;
use std::sync::Mutex;

use common::profile::{Sync, UserContext, UserProfile};
use tokio::fs;

use crate::dataans::DataansError;

pub struct WebService {
    user_data_dir: PathBuf,
    user_profile: Mutex<Option<UserProfile>>,
}

impl WebService {
    pub async fn new(user_data_dir: PathBuf) -> Result<Self, DataansError> {
        let profile_path = user_data_dir.join("profile.json");

        let user_profile = if profile_path.exists() {
            Some(serde_json::from_slice(&fs::read(profile_path).await?)?)
        } else {
            None
        };

        Ok(Self {
            user_data_dir,
            user_profile: Mutex::new(user_profile),
        })
    }

    pub async fn authorize(&self, profile: UserProfile) -> Result<(), DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        fs::write(profile_path, serde_json::to_vec(&profile)?).await?;

        let mut user_profile = self.user_profile.lock().unwrap();
        *user_profile = Some(profile);

        Ok(())
    }

    pub async fn load_user_profile(&self) -> Result<UserProfile, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        if !profile_path.exists() {
            return Err(DataansError::UserNotSignedIn);
        }

        Ok(serde_json::from_slice(&fs::read(&profile_path).await?)?)
    }

    pub fn user_profile(&self) -> Option<UserProfile> {
        self.user_profile.lock().unwrap().clone()
    }

    pub async fn user_context(&self) -> Result<Option<UserContext>, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        if profile_path.exists() {
            let UserProfile {
                sync_config,
                auth_token: _,
                secret_key: _,
                salt: _,
            } = serde_json::from_slice(&fs::read(profile_path).await?)?;

            Ok(Some(UserContext { sync_config }))
        } else {
            Ok(None)
        }
    }

    pub async fn set_sync_options(&self, sync_config: Sync) -> Result<UserContext, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        let mut user_profile: UserProfile = serde_json::from_slice(&fs::read(&profile_path).await?)?;
        user_profile.sync_config = sync_config;

        fs::write(profile_path, serde_json::to_vec(&user_profile)?).await?;

        let user_context = UserContext {
            sync_config: user_profile.sync_config.clone(),
        };

        *self.user_profile.lock().unwrap() = Some(user_profile);

        Ok(user_context)
    }
}
