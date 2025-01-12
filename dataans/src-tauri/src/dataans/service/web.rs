use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use common::profile::{SecretKey, UserProfile};
use rand::rngs::OsRng;
use rand::Rng;
use reqwest::Client;
use url::Url;
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, SignInRequest, SignInResponse, SignUpRequest, Username};

use crate::dataans::DataansError;

pub struct WebService {
    user_data_dir: PathBuf,
    web_server: Url,
    user_profile: Mutex<Option<UserProfile>>,
}

impl WebService {
    pub fn new(user_data_dir: PathBuf, web_server: Url) -> Self {
        Self {
            user_data_dir,
            user_profile: Mutex::new(None),
            web_server,
        }
    }

    pub async fn sign_up(
        &self,
        invitation_token: InvitationToken,
        username: Username,
        password: Password,
    ) -> Result<Uuid, DataansError> {
        let response = Client::new()
            .post(self.web_server.join("auth/sign-up")?)
            .json(&SignUpRequest {
                invitation_token,
                username,
                password,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DataansError::SignUpFailed(response.status()));
        }

        let user_id = response.json::<Uuid>().await?;
        let secret_key = SecretKey::from(OsRng.gen::<[u8; 32]>().to_vec());

        fs::write(
            self.user_data_dir.join(format!("{}.json", user_id)),
            hex::encode(secret_key.as_ref()),
        )?;

        Ok(user_id)
    }

    pub async fn sign_in(
        &self,
        secret_key: Option<SecretKey>,
        username: Username,
        password: Password,
    ) -> Result<(), DataansError> {
        let response = Client::new()
            .post(self.web_server.join("auth/sign-in")?)
            .json(&SignInRequest {
                username: username.clone(),
                password,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DataansError::SignUpFailed(response.status()));
        }

        let SignInResponse {
            user_id,
            token,
            expiration_date,
        } = response.json::<SignInResponse>().await?;

        let secret_key = if let Some(key) = secret_key {
            key
        } else {
            let secret_key_file_path = self.user_data_dir.join(format!("{}.json", user_id));
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
        };

        fs::write(
            self.user_data_dir.join("profile.json"),
            serde_json::to_vec(&user_profile)?,
        )?;

        *self.user_profile.lock().unwrap() = Some(user_profile);

        Ok(())
    }
}
