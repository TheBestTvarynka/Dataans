#![allow(dead_code)]
#![allow(unused_imports)]

mod client;
mod hash;

pub use hash::{Hasher, Hash};

use std::sync::Arc;

use sha2::{Sha256, Digest};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use web_api_types::{AuthToken, UserId, AUTH_HEADER_NAME};

use crate::dataans::crypto::{decrypt, encrypt, CryptoError, EncryptionKey};
use crate::dataans::db::{Db, DbError, Note as NoteModel, OperationDb, Space as SpaceModel};
use crate::dataans::service::note::NoteServiceError;
use crate::dataans::service::space::SpaceServiceError;
use crate::dataans::sync::client::Client;
use crate::dataans::{NoteService, SpaceService};

const OPERATIONS_PER_BLOCK: usize = 16;

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

pub async fn sync_future<D: Db + OperationDb>(
    db: Arc<D>,
    sync_server: Url,
    auth_token: AuthToken,
    encryption_key: EncryptionKey,
) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server, auth_token, encryption_key)?;

    synchronizer.synchronize().await
}

struct Synchronizer<D> {
    db: Arc<D>,
    encryption_key: EncryptionKey,
    client: Client,
}

impl<D: Db + OperationDb> Synchronizer<D> {
    pub fn new(
        db: Arc<D>,
        sync_server: Url,
        auth_token: AuthToken,
        encryption_key: EncryptionKey,
    ) -> Result<Self, SyncError> {
        Ok(Self {
            db,
            client: Client::new(sync_server, auth_token)?,
            encryption_key,
        })
    }

    async fn synchronize(&self) -> Result<(), SyncError> {
        let (local_operations, remote_blocks) = futures::join!(
            self.db.operations(),
            self.client.blocks(OPERATIONS_PER_BLOCK),
        );

        let local_blocks = local_operations?
            .chunks(OPERATIONS_PER_BLOCK)
            .map(|operations| {
                let mut hasher = Sha256::new();

                for operation in operations {
                    operation.hash(&mut hasher);
                }

                hasher.finalize().to_vec()
            })
            .collect::<Vec<_>>();

        trace!(?local_blocks, ?remote_blocks, "Syncing blocks");

        //

        Ok(())
    }
}
