use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures::future::try_join_all;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use web_api_types::{AuthToken, BlockId, BlockIds, BlockNotes, SyncBlock, AUTH_HEADER_NAME};

use crate::dataans::db::{Db, DbError, SyncBlock as SyncBlockModel};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("DB error: {0}")]
    DbError(#[from] DbError),

    #[error("failed to send a request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to parse a url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("local database is corrupted: non existent flocks found")]
    InvalidLocalBlocks(Vec<SyncBlockModel>),

    #[error(transparent)]
    InvalidHttpHeader(#[from] reqwest::header::InvalidHeaderValue),
}

pub async fn sync_future<D: Db>(db: Arc<D>, sync_server_url: Url, auth_token: AuthToken) -> Result<(), SyncError> {
    let synchronizer = Synchronizer::new(db, sync_server_url, auth_token)?;

    Ok(())
}

struct Synchronizer<D> {
    db: Arc<D>,
    client: Client,
    sync_server_url: Url,
}

impl<D: Db> Synchronizer<D> {
    pub fn new(db: Arc<D>, sync_server_url: Url, auth_token: AuthToken) -> Result<Self, SyncError> {
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
                        .join("block")?
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
                return Err(SyncError::InvalidLocalBlocks(
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
            .post(self.sync_server_url.join("block/notes")?)
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
        let mut notes_to_download = Vec::new();

        for (note_id, note_checksum) in server_notes {
            if let Some(local_note) = local_notes.remove(&note_id) {
                if local_note.checksum != note_checksum {
                    notes_to_sync.push(note_id);
                }
            } else {
                notes_to_download.push(note_id);
            }
        }

        let notes_to_upload = local_notes.keys().collect::<Vec<_>>();

        Ok(())
    }

    async fn sync_notes(&self, notes: &[Uuid]) -> Result<(), SyncError> {
        Ok(())
    }
    
    async fn upload_notes(&self, notes: &[Uuid]) -> Result<(), SyncError> {
        Ok(())
    }

    async fn download_notes(&self, notes: &[Uuid]) -> Result<(), SyncError> {
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
