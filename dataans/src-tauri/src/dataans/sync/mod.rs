use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use common::note::OwnedNote;
use futures::future::try_join_all;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use web_api_types::{
    AuthToken, BlockId, BlockIds, BlockNotes, Note as ServerNote, NoteIds, SyncBlock, AUTH_HEADER_NAME,
};

use crate::dataans::crypto::{decrypt, encrypt, CryptoError, EncryptionKey};
use crate::dataans::db::{Db, DbError, Note as NoteModel, SyncBlock as SyncBlockModel, SyncBlockNote};
use crate::dataans::service::note::NoteServiceError;
use crate::dataans::NoteService;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("DB error: {0}")]
    DbError(#[from] DbError),

    #[error("failed to send a request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to parse a url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("local or server database is corrupted: non existent flocks found")]
    InvalidSyncBlocks(Vec<SyncBlockModel>),

    #[error(transparent)]
    InvalidHttpHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error("invalid note ({0}): block id is missing: trying to sync not-uploaded note")]
    InvalidNoteBlockId(Uuid),

    #[error(transparent)]
    NoteService(#[from] NoteServiceError),
}

pub async fn sync_future<D: Db>(
    db: Arc<D>,
    sync_server_url: Url,
    auth_token: AuthToken,
    encryption_key: EncryptionKey,
) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server_url, auth_token, encryption_key)?;

    Ok(())
}

struct Synchronizer<D> {
    db: Arc<D>,
    client: Client,
    sync_server_url: Url,
    encryption_key: EncryptionKey,
}

impl<D: Db> Synchronizer<D> {
    pub fn new(
        db: Arc<D>,
        sync_server_url: Url,
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
            db,
            sync_server_url,
            client,
            encryption_key,
        })
    }

    pub async fn sync_full(&self) -> Result<(), SyncError> {
        // TODO: run these features in parallel (`join_all`).
        for space in self.db.spaces().await? {
            // TODO: run these features in parallel (`join_all`).
            let blocks = self
                .client
                .get(
                    self.sync_server_url
                        .join("/sync/block")?
                        .join(space.id.to_string().as_str())?,
                )
                .send()
                .await?
                .json::<Vec<SyncBlock>>()
                .await?;
            let local_blocks = self.db.blocks().await?;

            trace!(?blocks, ?local_blocks);

            let server_blocks = blocks
                .into_iter()
                .map(|block| (block.id, block))
                .collect::<HashMap<_, _>>();
            let mut local_blocks = local_blocks
                .into_iter()
                .map(|block| (block.id, block))
                .collect::<HashMap<_, _>>();

            let mut blocks_to_sync = Vec::new();

            for (block_id, block) in server_blocks {
                if let Some(local_block) = local_blocks.remove(block_id.as_ref()) {
                    if &local_block.checksum != block.checksum.as_ref() {
                        blocks_to_sync.push(block_id);
                    }
                } else {
                    blocks_to_sync.push(block_id);
                }
            }

            if !local_blocks.is_empty() {
                // Local blocks must not contain non-existent blocks. Every new block is created by the server during note uploading.
                // So, all local blocks must exist on the server side too.
                return Err(SyncError::InvalidSyncBlocks(
                    local_blocks.into_iter().map(|(_, block)| block).collect(),
                ));
            }

            self.sync_blocks(blocks_to_sync).await?;
        }

        Ok(())
    }

    async fn sync_blocks(&self, blocks: Vec<BlockId>) -> Result<(), SyncError> {
        // TODO: Run these features in parallel (`join_all`).
        let server_notes = self
            .client
            .post(self.sync_server_url.join("/sync/block/notes")?)
            .json(&BlockIds { ids: blocks.clone() })
            .send()
            .await?
            .json::<Vec<BlockNotes>>()
            .await?
            .into_iter()
            .flat_map(|block_notes| block_notes.notes.into_iter())
            .map(|note| (note.id.into(), note.checksum.into()))
            .collect::<HashMap<Uuid, Vec<u8>>>();
        let mut local_notes = try_join_all(blocks.into_iter().map(|block_id| self.db.block_notes(block_id.into())))
            .await?
            .into_iter()
            .flat_map(|block_notes| block_notes.into_iter().map(|note| (note.id, note)))
            .collect::<HashMap<_, _>>();

        trace!(?server_notes, ?local_notes);

        let mut notes_to_sync = Vec::new();

        for (note_id, note_checksum) in server_notes {
            if let Some(local_note) = local_notes.remove(&note_id) {
                if local_note.checksum != note_checksum {
                    notes_to_sync.push(note_id);
                }
            } else {
                notes_to_sync.push(note_id);
            }
        }

        // TODO: run these features in parallel (`join_all`).
        if !local_notes.is_empty() {
            // It means that some other client deleted some notes and synced it with the server.
            // Now we need to delete these notes from the local database too.
            try_join_all(local_notes.into_iter().map(|(note_id, _)| self.db.remove_note(note_id))).await?;
        }
        self.sync_notes(notes_to_sync).await?;

        Ok(())
    }

    async fn sync_notes(&self, notes: Vec<Uuid>) -> Result<(), SyncError> {
        // TODO: run these features in parallel (`join_all`).
        let server_notes = self
            .client
            .post(self.sync_server_url.join("/data/notes")?)
            .json(&NoteIds {
                ids: notes.iter().map(|id| (*id).into()).collect(),
            })
            .send()
            .await?
            .json::<Vec<ServerNote>>()
            .await?
            .into_iter()
            .map(|note| {
                let OwnedNote {
                    id,
                    text,
                    created_at,
                    updated_at,
                    space_id,
                    files,
                } = decrypt(note.data.as_ref(), &self.encryption_key)?;
                Ok((
                    note.id.into(),
                    NoteModel {
                        id: id.into(),
                        text: text.into(),
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        space_id: space_id.into(),
                        block_id: Some(note.block_id.into()),
                        checksum: note.checksum.into(),
                    },
                ))
            })
            .collect::<Result<HashMap<Uuid, NoteModel>, CryptoError>>()?;
        let mut local_notes = try_join_all(notes.into_iter().map(|note_id| self.db.note_by_id(note_id)))
            .await?
            .into_iter()
            .map(|note| (note.id, note))
            .collect::<HashMap<Uuid, NoteModel>>();

        // TODO: run these features in parallel (`join_all`).
        for (note_id, server_note) in server_notes {
            if let Some(local_note) = local_notes.remove(&note_id) {
                if server_note.checksum != local_note.checksum {
                    // We have a conflict. Server and local databases contain different versions of the same note.
                    // Our current strategy is to prefer a note with the latest update time.
                    if server_note.updated_at > local_note.updated_at {
                        self.db.update_note(&server_note).await?;
                    } else {
                        self.update_server_note(local_note).await?;
                    }
                }
            } else {
                // The server has a new note the client doesn't have.
                self.db.create_note(&server_note).await?;
            }
        }

        if !local_notes.is_empty() {
            warn!("Something weird happens here. Such a case should not be possible.");
        }

        Ok(())
    }

    async fn update_server_note(&self, note: NoteModel) -> Result<(), SyncError> {
        let checksum = note.checksum.clone();
        let block_id = note.block_id.ok_or(SyncError::InvalidNoteBlockId(note.id))?;
        let note = NoteService::map_note_model_to_note(note, &*self.db).await?;
        let encrypted_note = encrypt(&note, &self.encryption_key)?;

        let server_notes = self
            .client
            .put(self.sync_server_url.join("/data/note")?)
            .json(&ServerNote {
                id: note.id.inner().into(),
                data: encrypted_note.into(),
                checksum: checksum.into(),
                space_id: note.space_id.inner().into(),
                block_id: block_id.into(),
            })
            .send()
            .await?;

        Ok(())
    }

    async fn upload_note(&self, note: NoteModel) -> Result<(), SyncError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sync() {
        //
    }
}
