use std::sync::Arc;

use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, Username};

use crate::db::{AuthDb, Session, User};
use crate::{crypto, Result, SERVER_ENCRYPTION_KEY_SIZE};

const SESSION_DURATION: Duration = Duration::days(30);

pub struct Auth<A> {
    encryption_key: [u8; SERVER_ENCRYPTION_KEY_SIZE],
    auth_db: Arc<A>,
}

impl<A: AuthDb> Auth<A> {
    pub fn new(auth_db: Arc<A>, encryption_key: [u8; SERVER_ENCRYPTION_KEY_SIZE]) -> Self {
        Self {
            auth_db,
            encryption_key,
        }
    }

    pub async fn sign_up(&self, token: InvitationToken, username: &Username, password: &Password) -> Result<Uuid> {
        let token = self.auth_db.find_invitation_token(token.as_ref()).await?;

        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: crypto::sha256(username.as_bytes()).into(),
            password: crypto::hash_password(password.as_bytes())?,
        };
        self.auth_db.add_user(&user, token.id).await?;

        Ok(user_id)
    }

    pub async fn sign_in(&self, username: &Username, password: &Password) -> Result<String> {
        let user = self
            .auth_db
            .find_user_by_username(crypto::sha256(username.as_bytes()).as_ref())
            .await?;

        crypto::verify_password(password.as_bytes(), &user.password)?;

        let now = OffsetDateTime::now_utc();
        let created_at = PrimitiveDateTime::new(now.date(), now.time());
        let session = Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            created_at: created_at.clone(),
            expiration_date: created_at + SESSION_DURATION,
        };
        self.auth_db.add_session(&session).await?;

        Ok(hex::encode(crypto::encrypt(
            session.id.as_bytes(),
            &self.encryption_key,
        )?))
    }
}
