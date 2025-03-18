use std::sync::Arc;

use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use web_api_types::{InvitationToken, Password, UserId, Username};

use crate::crypto::EncryptionKey;
use crate::db::{AuthDb, DbError, Session, User};
use crate::{crypto, Error, Result};

const SESSION_DURATION: Duration = Duration::days(30);

pub struct Auth<A> {
    encryption_key: EncryptionKey,
    auth_db: Arc<A>,
}

impl<A: AuthDb> Auth<A> {
    pub fn new(auth_db: Arc<A>, encryption_key: EncryptionKey) -> Self {
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

    pub async fn sign_in(&self, username: &Username, password: &Password) -> Result<(UserId, String, OffsetDateTime)> {
        let user = self
            .auth_db
            .find_user_by_username(crypto::sha256(username.as_bytes()).as_ref())
            .await?;

        crypto::verify_password(password.as_bytes(), &user.password)?;

        let created_at = OffsetDateTime::now_utc();
        let expiration_date = created_at + SESSION_DURATION;

        let session = Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            created_at,
            expiration_date,
        };
        self.auth_db.add_session(&session).await?;

        let token = hex::encode(crypto::encrypt(session.id.as_bytes(), &self.encryption_key)?);
        Ok((user.id.into(), token, expiration_date))
    }

    pub async fn verify_session(&self, token: &str) -> Result<UserId> {
        let token = crypto::decrypt(
            &hex::decode(token).map_err(|_err| Error::Session("invalid auth token"))?,
            &self.encryption_key,
        )?;

        let session_id = Uuid::from_slice(&token).map_err(|err| {
            error!(
                ?err,
                "Failed to construct session id from decrypted token. Possible internal data corruption."
            );

            Error::Session("invalid token")
        })?;

        let session = self.auth_db.session(session_id).await.map_err(|err| {
            if let DbError::SqlxError(sqlx::Error::RowNotFound) = err {
                Error::Session("not found")
            } else {
                err.into()
            }
        })?;

        if session.expiration_date < OffsetDateTime::now_utc() {
            return Err(Error::Session("expired"));
        }

        Ok(session.user_id.into())
    }
}
