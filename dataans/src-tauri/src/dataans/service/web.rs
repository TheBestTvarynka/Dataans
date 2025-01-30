use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use common::profile::{SecretKey, Sync, UserContext, UserProfile, WebServerUrl};
use rand::rngs::OsRng;
use rand::Rng;
use reqwest::Client;
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, SignInRequest, SignInResponse, SignUpRequest, Username};

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

    pub async fn sign_up(
        &self,
        invitation_token: InvitationToken,
        username: Username,
        password: Password,
        web_server_url: WebServerUrl,
    ) -> Result<Uuid, DataansError> {
        let response = Client::new()
            .post(web_server_url.as_ref().join("auth/sign-up")?)
            .json(&SignUpRequest {
                invitation_token,
                username,
                password,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_msg = response.text().await?;
            return Err(DataansError::SignUpFailed(err_msg));
        }

        let user_id = response.json::<Uuid>().await?;
        let secret_key = SecretKey::from(OsRng.gen::<[u8; 32]>().to_vec());

        fs::write(
            self.user_data_dir.join(format!("secret-key-{}.json", user_id)),
            hex::encode(secret_key.as_ref()),
        )?;

        Ok(user_id)
    }

    pub async fn sign_in(
        &self,
        secret_key: Option<SecretKey>,
        username: Username,
        password: Password,
        web_server_url: WebServerUrl,
    ) -> Result<UserContext, DataansError> {
        let response = Client::new()
            .post(web_server_url.as_ref().join("auth/sign-in")?)
            .json(&SignInRequest {
                username: username.clone(),
                password,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_msg = response.text().await?;
            return Err(DataansError::SignInFailed(err_msg));
        }

        let SignInResponse {
            user_id,
            token,
            expiration_date,
        } = response.json::<SignInResponse>().await?;

        let secret_key = if let Some(key) = secret_key {
            key
        } else {
            let secret_key_file_path = self.user_data_dir.join(format!("secret-key-{}.json", user_id.as_ref()));
            SecretKey::from(
                hex::decode(
                    fs::read(&secret_key_file_path)
                        .map_err(|err| DataansError::SecretKeyFile(secret_key_file_path, err))?,
                )
                .map_err(|err| DataansError::ParseSecretKey(err))?,
            )
        };

        let user_profile = UserProfile {
            user_id,
            username,
            auth_token: token,
            auth_token_expiration_date: expiration_date,
            secret_key,
            sync_config: Sync::Disabled { url: web_server_url },
        };

        fs::write(
            self.user_data_dir.join("profile.json"),
            serde_json::to_vec(&user_profile)?,
        )?;

        let user_context = UserContext {
            user_id: user_profile.user_id,
            username: user_profile.username.clone(),
            sync_config: user_profile.sync_config.clone(),
        };

        *self.user_profile.lock().unwrap() = Some(user_profile);

        Ok(user_context)
    }

    pub fn user_context(&self) -> Result<Option<UserContext>, DataansError> {
        let profile_path = self.user_data_dir.join("profile.json");

        if profile_path.exists() {
            let UserProfile {
                user_id,
                username,
                sync_config,
                auth_token: _,
                auth_token_expiration_date: _,
                secret_key: _,
            } = serde_json::from_slice(&fs::read(profile_path)?)?;

            Ok(Some(UserContext {
                user_id,
                username,
                sync_config,
            }))
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
            user_id: user_profile.user_id,
            username: user_profile.username.clone(),
            sync_config: user_profile.sync_config.clone(),
        };

        *self.user_profile.lock().unwrap() = Some(user_profile);

        Ok(user_context)
    }
}
