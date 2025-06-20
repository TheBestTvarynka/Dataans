#![allow(dead_code)]
#![allow(unused_imports)]

mod client;
mod hash;

use std::sync::Arc;

use common::event::DATA_EVENT;
pub use hash::{Hash, Hasher};
use sha2::{Digest, Sha256};
use tauri::{Emitter, Runtime};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use web_api_types::{AuthToken, UserId, AUTH_HEADER_NAME};

use crate::dataans::crypto::{decrypt, encrypt, CryptoError, EncryptionKey};
use crate::dataans::db::{
    Db, DbError, Note as NoteModel, Operation, OperationDb, OperationRecord, OperationRecordOwned, Space as SpaceModel,
};
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

    #[error("sync failed: {0}")]
    SyncFailed(&'static str),

    #[error("failed to send event: {0}")]
    Event(&'static str),
}

#[instrument(ret, skip(db, auth_token, encryption_key, emitter))]
pub async fn sync_future<D: OperationDb, R: Runtime, E: Emitter<R>>(
    db: Arc<D>,
    sync_server: Url,
    auth_token: AuthToken,
    encryption_key: EncryptionKey,
    emitter: E,
) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server, auth_token, encryption_key)?;

    synchronizer.synchronize(emitter).await
}

struct Synchronizer<D> {
    db: Arc<D>,
    client: Client,
}

impl<D: OperationDb> Synchronizer<D> {
    pub fn new(
        db: Arc<D>,
        sync_server: Url,
        auth_token: AuthToken,
        encryption_key: EncryptionKey,
    ) -> Result<Self, SyncError> {
        Ok(Self {
            db,
            client: Client::new(sync_server, auth_token, encryption_key)?,
        })
    }

    #[instrument(err, skip(self, emitter))]
    async fn synchronize<R: Runtime, E: Emitter<R>>(&self, emitter: E) -> Result<(), SyncError> {
        let (local_operations, remote_blocks) =
            futures::join!(self.db.operations(), self.client.blocks(OPERATIONS_PER_BLOCK),);

        let mut local_operations = local_operations?;
        local_operations.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        let remote_blocks = remote_blocks?;

        let local_blocks = local_operations
            .chunks(OPERATIONS_PER_BLOCK)
            .map(|operations| {
                let mut hasher = Sha256::new();

                for operation in operations {
                    hasher.update(operation.digest::<Sha256>());
                }

                hasher.finalize().to_vec()
            })
            .collect::<Vec<_>>();

        trace!(?local_blocks, ?remote_blocks, "Syncing blocks");

        let mut blocks_to_skip = 0;

        while let (Some(local_hash), Some(remote_hash)) =
            (local_blocks.get(blocks_to_skip), remote_blocks.get(blocks_to_skip))
        {
            if local_hash == remote_hash {
                blocks_to_skip += 1;
            } else {
                break;
            }
        }

        if local_blocks.len() == remote_blocks.len() && local_blocks.len() == blocks_to_skip {
            info!("Nothing to sync, all blocks are equal.");
            return Ok(());
        }

        let mut local_operations = local_operations[blocks_to_skip * OPERATIONS_PER_BLOCK..].iter();
        let remote_operations = self.client.operations(blocks_to_skip * OPERATIONS_PER_BLOCK).await?;
        let mut remote_operations = remote_operations.iter();

        let mut operations_to_upload = Vec::new();
        let mut operations_to_apply = Vec::new();

        loop {
            match (local_operations.next(), remote_operations.next()) {
                (Some(local_operation), Some(remote_operation)) => {
                    if local_operation.id != remote_operation.id {
                        operations_to_upload.push(local_operation);
                        operations_to_apply.push(remote_operation);

                        for local_operation in local_operations {
                            operations_to_upload.push(local_operation);
                        }

                        for remote_operation in remote_operations {
                            operations_to_apply.push(remote_operation);
                        }

                        break;
                    }
                }
                (Some(local_operation), None) => {
                    operations_to_upload.push(local_operation);
                }
                (None, Some(remote_operation)) => {
                    operations_to_apply.push(remote_operation);
                }
                (None, None) => break,
            }
        }

        trace!(?operations_to_upload);
        trace!(?operations_to_apply);

        let result = futures::join!(
            async {
                for operation in operations_to_apply {
                    if let Some(event) = self.db.apply_operation(operation).await? {
                        emitter.emit(DATA_EVENT, event).map_err(|err| {
                            error!(?err, "Failed to emit data event");
                            SyncError::Event("failed to emit data event")
                        })?;
                    }
                }

                Result::<_, SyncError>::Ok(())
            },
            self.client.upload_operations(&operations_to_upload)
        );

        match result {
            (Ok(_), Ok(_)) => {
                info!("Synchronization successful.");

                Ok(())
            }
            (Err(apply_err), Err(upload_err)) => {
                error!(
                    ?apply_err,
                    ?upload_err,
                    "Failed to apply remote operations and upload local operations"
                );

                Err(SyncError::SyncFailed(
                    "failed to apply remote operations and upload local operations",
                ))
            }
            (Err(err), _) => {
                error!(?err, "Failed to apply remote operations");

                Err(SyncError::SyncFailed("failed to apply remote operations"))
            }
            (_, Err(err)) => {
                error!(?err, "Failed to upload remote operations");

                Err(SyncError::SyncFailed("failed to upload remote operations"))
            }
        }
    }
}
