#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use web_api_types::{AuthToken, UserId, AUTH_HEADER_NAME};

use crate::dataans::crypto::{decrypt, encrypt, CryptoError, EncryptionKey};
use crate::dataans::db::{Db, DbError, Note as NoteModel, Space as SpaceModel};
use crate::dataans::service::note::NoteServiceError;
use crate::dataans::service::space::SpaceServiceError;
use crate::dataans::{NoteService, SpaceService};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("DB error: {0}")]
    DbError(#[from] DbError),

    #[error("failed to send a request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to parse a url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    InvalidHttpHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error("invalid note ({0}): block id is missing: trying to sync not-uploaded note")]
    InvalidNoteBlockId(Uuid),

    #[error(transparent)]
    NoteService(#[from] NoteServiceError),

    #[error(transparent)]
    SpaceService(#[from] SpaceServiceError),
}

pub async fn sync_future<D: Db>(
    user_id: UserId,
    db: Arc<D>,
    sync_server: Url,
    auth_token: AuthToken,
    encryption_key: EncryptionKey,
) -> Result<(), SyncError> {
    let _synchronizer = Synchronizer::new(user_id, db, sync_server, auth_token, encryption_key)?;

    // TODO

    Ok(())
}

struct Synchronizer<D> {
    user_id: UserId,
    db: Arc<D>,
    client: Client,
    sync_server: Url,
    encryption_key: EncryptionKey,
}

impl<D: Db> Synchronizer<D> {
    pub fn new(
        user_id: UserId,
        db: Arc<D>,
        sync_server: Url,
        auth_token: AuthToken,
        encryption_key: EncryptionKey,
    ) -> Result<Self, SyncError> {
        let client = ClientBuilder::new()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(AUTH_HEADER_NAME, HeaderValue::from_str(auth_token.as_ref())?);
                headers
            })
            .http2_keep_alive_interval(Some(Duration::from_secs(30)))
            .http2_keep_alive_timeout(Duration::from_secs(30))
            .http2_keep_alive_while_idle(true)
            .build()?;

        Ok(Self {
            user_id,
            db,
            sync_server,
            client,
            encryption_key,
        })
    }
}
