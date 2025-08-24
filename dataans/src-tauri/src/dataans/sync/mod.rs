//! The data synchronization algorithm implementation.
//!
//! # How it works
//!
//! ## General principle
//!
//! The user has some local database state. Alongside user's data, the app also
//! tracks all user operations like note creation, updating, space deletion, etc.
//! During the sync process all these operations are synchronized.
//!
//! After some time, the remote and local states may be outdated. The server may have
//! operations made by the user on other devices, and the local database may have
//! some operations that have not been uploaded to the remote server. When these two sets
//! are determined, then the app applies the needed remote operations on the local database
//! and uploads the needed operations to the server.
//!
//! ## Conflict resolution strategy
//!
//! Every object (like a note, a space, or a file) has a corresponding timestamp. During
//! the operation applying, the object with the latest timestamp wins (i.e. last write wins).
//!
//! ## Sync algorithm
//!
//! ### Blocks
//!
//! The naive approach would be to request all operations the server has, and then compare them
//! to local ones, and find the difference. But there can be a lot of operations. So, there is
//! a small optimization: _synchronization blocks_ (or just _blocks_).
//!
//! Synchronization blocks is a hash of the N continuous operations hashes (e.g.
//! `let block = hash(hash(operations[0]) | ... | hash(operations[N - 1]));`).
//!
//! If the server and client have some block with the same hash, then there is no need to sync
//! operations that belong to this block. They are also equal.
//!
//! ### Algorithm
//!
//! **Step 1.** The app requests server's blocks and calculates local blocks.
//!
//! **Step 2.** The app determines the same blocks in two lists. It compares them one by one until
//! the first pair of blocks with different hashes.
//!
//! **Step 3.** Now the app knows the amount of the common blocks and can skip operations that belong
//! to these blocks. The app skips them and requests all other operations that the server has.
//!
//! **Step 4.** The app has two sets of operations: local one and remote one. It finds the difference
//! between them. The first set will contain operations to upload and the second one will contain
//! operations to apply.
//!
//! **Step 5.** The app uploads and applies operations from the corresponding sets.
//!
//! Basically, that's all.
//!
//! ### Files
//!
//! When the sync process starts, the synchronizer iterates over all files registered in the local
//! database and determines which ones need to be uploaded/downloaded.
//!
//! The main sync and files sync tasks communicate over the channel. If any remote operation
//! introduces a new file, then the main sync task will inform the file sync task about it. In turn,
//! the file sync task will download this file.
//!
//! The synchronization process finishes only when both main and file sync futures are completed.

pub mod client;
mod hash;

use std::path::Path;
use std::sync::Arc;

use common::event::{DATA_EVENT, DataEvent};
use common::note::{FileId, FileStatus};
use common::profile::AuthorizationToken;
use futures::StreamExt;
use futures::stream::FuturesUnordered;
pub use hash::{Hash, Hasher};
use sha2::{Digest, Sha256};
use tauri::async_runtime::{Receiver, Sender, channel};
use tauri::{Emitter, Runtime};
use thiserror::Error;
use tokio_stream::wrappers::ReceiverStream;
use url::Url;
use uuid::Uuid;

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

    #[error("health check failed: {0:?}")]
    HealthCheckFailed(reqwest::StatusCode),
}

#[instrument(err, skip(db, encryption_key, emitter))]
pub async fn sync_future<D: OperationDb, R: Runtime, E: Emitter<R>>(
    db: Arc<D>,
    sync_server: Url,
    encryption_key: EncryptionKey,
    emitter: &E,
    files_path: Arc<Path>,
    auth_token: &AuthorizationToken,
) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server, encryption_key, files_path, auth_token)?;

    let (sender, receiver) = channel::<FileId>(CHANNEL_BUFFER_SIZE);

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

/// Does all the synchronization work.
struct Synchronizer<D> {
    db: Arc<D>,
    client: Client,
    files_path: Arc<Path>,
}

impl<D: OperationDb> Synchronizer<D> {
    /// Created a new [Synchronizer] instance.
    pub fn new(
        db: Arc<D>,
        sync_server: Url,
        encryption_key: EncryptionKey,
        files_path: Arc<Path>,
        auth_token: &AuthorizationToken,
    ) -> Result<Self, SyncError> {
        Ok(Self {
            db,
            client: Client::new(sync_server, encryption_key, auth_token)?,
            files_path,
        })
    }

    /// This function takes the `file_id` and determines what we need to do to this file.
    ///
    /// If the file needs to be uploaded, then it will upload it.
    /// If the file needs to be downloaded, then it will download it.
    ///
    /// This function automatically sends update event to the frontend using the provided `emitter`.
    async fn handle_file<R: Runtime, E: Emitter<R>>(&self, file_id: Uuid, emitter: &E) -> Result<(), SyncError> {
        let file = self.db.file_by_id(*file_id.as_ref()).await?;
        let file_path = self.files_path.join(&file.path);

        if file.is_uploaded {
            if !file_path.exists() {
                debug!(?file.id, ?file.path, "File does not exist locally, but is uploaded. Downloading...");

                self.client.download_file(file.id, &file_path).await?;
                emitter
                    .emit(
                        DATA_EVENT,
                        DataEvent::FileStatusUpdated(file.id.into(), FileStatus::ExistAndUploaded),
                    )
                    .map_err(|err| {
                        error!(?err, "Failed to emit data event");
                        SyncError::Event("failed to emit data event")
                    })?;
            } else {
                debug!(?file.id, ?file.path, "File exists locally and is uploaded. Nothing to do.");
            }
        } else if file_path.exists() {
            debug!(?file.id, ?file.path, "File exists locally, but is not uploaded. Uploading...");

            self.client.upload_file(file.id, &file_path).await?;
            self.db.mark_file_as_uploaded(file.id).await?;
            emitter
                .emit(
                    DATA_EVENT,
                    DataEvent::FileStatusUpdated(file.id.into(), FileStatus::ExistAndUploaded),
                )
                .map_err(|err| {
                    error!(?err, "Failed to emit data event");
                    SyncError::Event("failed to emit data event")
                })?;
        } else {
            warn!(?file.id, ?file.path, "File does not exist locally and is not uploaded. Something weird happens here...");
            emitter
                .emit(
                    DATA_EVENT,
                    DataEvent::FileStatusUpdated(file.id.into(), FileStatus::NotExistAndNotUploaded),
                )
                .map_err(|err| {
                    error!(?err, "Failed to emit data event");
                    SyncError::Event("failed to emit data event")
                })?;
        }

        Ok(())
    }

    /// Synchronizes the files between local user's machine and the remote server.
    ///
    /// It iterates over all user files and downloads/uploads them if needed. Moreover,
    /// If there are new files discovered during the data synchronization, it will download them.
    async fn synchronize_files<R: Runtime, E: Emitter<R>>(
        &self,
        emitter: &E,
        receiver: Receiver<FileId>,
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
                        let fut = self.handle_file(file_id.into(), emitter);
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

        let mut result = Ok(());

        while let Some(task_result) = tasks.next().await {
            debug!(?task_result, "File synchronization task finished");
            if let Err(err) = task_result {
                result = Err(err);
            }
        }

        result
    }

    /// Does local and remote databases synchronization.
    #[instrument(err, skip(self, emitter))]
    async fn synchronize<R: Runtime, E: Emitter<R>>(
        &self,
        emitter: &E,
        sender: Sender<FileId>,
    ) -> Result<(), SyncError> {
        // Step 1: calculate local blocks and request server blocks.
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

        // Step 2: Determine shared blocks.
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
        // Step 3: Request operations from the server.
        let remote_operations = self.client.operations(blocks_to_skip * OPERATIONS_PER_BLOCK).await?;
        let mut remote_operations = remote_operations.iter();

        // Step 4: Find the difference between local and remote operations.
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

        // Step 5: Upload local operations that the server does not have and apply remote operations
        // on the local database that the current user does not have.
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
