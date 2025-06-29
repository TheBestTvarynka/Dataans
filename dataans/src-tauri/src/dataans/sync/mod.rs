mod client;
mod hash;

use std::path::Path;
use std::sync::Arc;

use common::event::{DataEvent, DATA_EVENT};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
pub use hash::{Hash, Hasher};
use sha2::{Digest, Sha256};
use tauri::async_runtime::{channel, Receiver, Sender};
use tauri::{Emitter, Runtime};
use thiserror::Error;
use tokio_stream::wrappers::ReceiverStream;
use url::Url;
use uuid::Uuid;
use web_api_types::AuthToken;

use crate::dataans::crypto::{CryptoError, EncryptionKey};
use crate::dataans::db::{DbError, OperationDb};
use crate::dataans::sync::client::Client;

const OPERATIONS_PER_BLOCK: usize = 16;
const CHANNEL_BUFFER_SIZE: usize = 64;

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

    #[error("sync failed: {0}")]
    SyncFailed(&'static str),

    #[error("failed to send event: {0}")]
    Event(&'static str),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[instrument(ret, skip(db, auth_token, encryption_key, emitter))]
pub async fn sync_future<D: OperationDb, R: Runtime, E: Emitter<R>>(
    db: Arc<D>,
    sync_server: Url,
    auth_token: AuthToken,
    encryption_key: EncryptionKey,
    emitter: &E,
) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server, auth_token, encryption_key)?;

    let (sender, receiver) = channel::<Uuid>(CHANNEL_BUFFER_SIZE);

    let main_sync_fut = synchronizer.synchronize(emitter, sender);
    let file_sync_fut = synchronizer.synchronize_files(emitter, receiver);

    let (main_result, file_result) = futures::join!(main_sync_fut, file_sync_fut);

    match (main_result, file_result) {
        (Ok(_), Ok(_)) => {
            info!("Synchronization successful.");

            Ok(())
        }
        (Err(main_err), Err(file_err)) => {
            error!(?main_err, ?file_err, "Failed to sync DB data and files data");

            Err(SyncError::SyncFailed("failed to sync DB data and files data"))
        }
        (Err(err), _) => {
            error!(?err, "Failed to sync DB data");

            Err(SyncError::SyncFailed("failed to sync DB data"))
        }
        (_, Err(err)) => {
            error!(?err, "Failed to sync files data");

            Err(SyncError::SyncFailed("failed to sync files data"))
        }
    }
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

    async fn handle_file<R: Runtime, E: Emitter<R>>(&self, file_id: Uuid, _emitter: &E) -> Result<(), SyncError> {
        let file = self.db.file_by_id(file_id).await?;
        let file_path = Path::new(&file.path);

        if file.is_uploaded {
            if !file_path.exists() {
                debug!(?file.id, ?file.path, "File does not exist locally, but is uploaded. Downloading...");

                self.client.download_file(file.id, file_path).await?;
            } else {
                debug!(?file.id, ?file.path, "File exists locally and is uploaded. Nothing to do.");
            }
        } else {
            if file_path.exists() {
                debug!(?file.id, ?file.path, "File exists locally, but is not uploaded. Uploading...");

                self.client.upload_file(file.id, file_path).await?;
                self.db.mark_file_as_uploaded(file.id).await?;
            } else {
                warn!(?file.id, ?file.path, "File does not exist locally and is not uploaded. Something weird happens here...");
            }
        }

        Ok(())
    }

    async fn synchronize_files<R: Runtime, E: Emitter<R>>(
        &self,
        emitter: &E,
        receiver: Receiver<Uuid>,
    ) -> Result<(), SyncError> {
        let mut tasks = self
            .db
            .files()
            .await?
            .into_iter()
            .map(|file| self.handle_file(file.id, emitter))
            .collect::<FuturesUnordered<_>>();

        let receiver_stream = ReceiverStream::new(receiver);
        let mut receiver_stream = receiver_stream.fuse();

        loop {
            futures::select! {
                file_id = receiver_stream.next() => {
                    if let Some(file_id) = file_id {
                        let fut = self.handle_file(file_id, emitter);
                        tasks.push(fut);
                    } else {
                        break;
                    }
                }
                task_result = tasks.next() => {
                    debug!(?task_result, "File synchronization task finished");
                }
            }
        }

        while let Some(task_result) = tasks.next().await {
            debug!(?task_result, "File synchronization task finished");
        }

        Ok(())
    }

    #[instrument(err, skip(self, emitter))]
    async fn synchronize<R: Runtime, E: Emitter<R>>(&self, emitter: &E, sender: Sender<Uuid>) -> Result<(), SyncError> {
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
                    if let Some(event) = self.db.apply_operation(operation).await.inspect_err(|err| {
                        error!(?err, ?operation, "Failed to apply operation");
                    })? {
                        if let DataEvent::FileAdded(file) = &event {
                            sender.send(file.id).await.map_err(|err| {
                                error!(?err, "Failed to send file id into the channel");
                                SyncError::Event("failed to send file id into the channel")
                            })?;
                        }
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
