use std::sync::Arc;

use common::profile::UserProfile;
use reqwest::Client;
use url::Url;
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, SignUpRequest, Username};

use crate::dataans::DataansError;

pub struct WebService {
    web_server: Url,
    user_profile: Option<UserProfile>,
}

impl WebService {
    pub fn new(web_server: Url) -> Self {
        Self {
            user_profile: None,
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

        Ok(response.json().await?)
    }

    pub async fn sign_in(&self, username: Username, password: Password) -> Result<(), DataansError> {
        // TODO
        Ok(())
    }
}
