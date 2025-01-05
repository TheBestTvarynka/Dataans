use std::sync::Arc;

use uuid::Uuid;
use web_api_types::{InvitationToken, Password, Username};

use crate::db::{AuthDb, DbError, User};
use crate::{crypto, Error, Result};

pub struct Auth<A> {
    auth_db: Arc<A>,
}

impl<A: AuthDb> Auth<A> {
    pub fn new(auth_db: Arc<A>) -> Self {
        Self { auth_db }
    }

    pub async fn sign_up(&self, token: InvitationToken, username: &Username, password: &Password) -> Result<Uuid> {
        let token = self
            .auth_db
            .find_invitation_token(token.as_ref())
            .await
            .map_err(|err| {
                if let DbError::SqlxError(sqlx::Error::RowNotFound) = &err {
                    Error::InvitationTokenNotFound(token)
                } else {
                    Error::from(err)
                }
            })?;

        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: crypto::sha256(username.as_bytes()).into(),
            password: crypto::hash_password(password.as_bytes())?,
        };
        self.auth_db.add_user(&user, token.id).await?;

        Ok(user_id)
    }
}
