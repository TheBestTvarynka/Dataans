use web_api_types::{InvitationToken, Password, Username};

use crate::Result;

pub struct Auth {}

impl Auth {
    pub async fn sign_up(&self, token: &InvitationToken, username: &Username, password: &Password) -> Result<()> {
        Ok(())
    }
}
