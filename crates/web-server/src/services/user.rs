use std::sync::Arc;

use web_api_types::User;

use crate::Result;
use crate::db::{DbError, User as UserModel, UserDb};

pub struct UserService<D> {
    db: Arc<D>,
}

impl<D> UserService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }
}

impl<D: UserDb> UserService<D> {
    pub async fn init(&self, user: User) -> Result<()> {
        let User { id, secret_key_hash } = user;

        self.db
            .init(&UserModel {
                id: id.into(),
                secret_key_hash: secret_key_hash.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn user(&self) -> Result<Option<User>> {
        match self.db.user().await {
            Ok(user) => {
                let UserModel { id, secret_key_hash } = user;

                Ok(Some(User {
                    id: id.into(),
                    secret_key_hash: secret_key_hash.into(),
                }))
            }
            Err(DbError::SqlxError(sqlx::Error::RowNotFound)) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
