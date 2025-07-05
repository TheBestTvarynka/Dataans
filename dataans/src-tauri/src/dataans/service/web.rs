use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use common::profile::{Sync, UserContext, UserProfile};

use crate::dataans::DataansError;

pub struct WebService {
    user_data_dir: PathBuf,
    user_profile: Mutex<Option<UserProfile>>,
}

impl WebService {
    pub fn new(user_data_dir: PathBuf) -> Result<Self, DataansError> {
        let profile_path = user_data_dir.join("profile.json");

        let user_profile = if profile_path.exists() {
            Some(serde_json::from_slice(&fs::read(profile_path)?)?)
        } else {
            None
        };

        Ok(Self {
            user_data_dir,
            user_profile: Mutex::new(user_profile),
        })
    }

    pub fn user_profile(&self) -> Option<UserProfile> {
        self.user_profile.lock().unwrap().clone()
    }

    pub fn user_context(&self) -> Result<Option<UserContext>, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        if profile_path.exists() {
            let UserProfile {
                sync_config,
                auth_token_expiration_date: _,
                secret_key: _,
            } = serde_json::from_slice(&fs::read(profile_path)?)?;

            Ok(Some(UserContext { sync_config }))
        } else {
            Ok(None)
        }
    }

    pub fn set_sync_options(&self, sync_config: Sync) -> Result<UserContext, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        let mut user_profile: UserProfile = serde_json::from_slice(&fs::read(&profile_path)?)?;
        user_profile.sync_config = sync_config;

        fs::write(profile_path, serde_json::to_vec(&user_profile)?)?;

        let user_context = UserContext {
            sync_config: user_profile.sync_config.clone(),
        };

        *self.user_profile.lock().unwrap() = Some(user_profile);

        Ok(user_context)
    }
}
