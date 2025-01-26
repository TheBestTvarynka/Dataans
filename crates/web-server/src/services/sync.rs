use std::sync::Arc;

use futures::future::try_join_all;
use web_api_types::{BlockId, BlockNotes, NoteChecksum, SpaceId, SyncBlock, UserId};

use crate::db::{NoteChecksum as ModelNoteChecksum, NoteDb, SpaceDb, SyncBlock as ModelSyncBlock, SyncDb};
use crate::{Error, Result};

pub struct Sync<A> {
    db: Arc<A>,
}

impl<A: NoteDb + SyncDb + SpaceDb> Sync<A> {
    pub fn new(db: Arc<A>) -> Self {
        Self { db }
    }

    pub async fn blocks(&self, space_id: SpaceId, user_id: UserId) -> Result<Vec<SyncBlock>> {
        let space = self.db.space(*space_id.as_ref()).await?;

        if space.user_id != *user_id.as_ref() {
            return Err(Error::AccessDenied);
        }

        Ok(self
            .db
            .blocks(*space_id.as_ref())
            .await?
            .into_iter()
            .map(|sync_block| {
                let ModelSyncBlock {
                    id,
                    number,
                    checksum,
                    space_id,
                } = sync_block;

                SyncBlock {
                    id: id.into(),
                    number: number.into(),
                    checksum: checksum.into(),
                    space_id: space_id.into(),
                }
            })
            .collect())
    }

    pub async fn blocks_notes(&self, blocks: &[BlockId], user_id: UserId) -> Result<Vec<BlockNotes>> {
        for block in blocks {
            let owner_id = self.db.block_owner(*block.as_ref()).await?;

            if owner_id != *user_id.as_ref() {
                return Err(Error::AccessDenied);
            }
        }

        let notes = try_join_all(blocks.iter().map(|block_id| async {
            let notes = self.db.block_notes(*block_id.as_ref()).await?;

            Result::<_>::Ok(BlockNotes {
                block_id: *block_id,
                notes: notes
                    .into_iter()
                    .map(|note| {
                        let ModelNoteChecksum { id, checksum } = note;

                        NoteChecksum {
                            id: id.into(),
                            checksum: checksum.into(),
                        }
                    })
                    .collect(),
            })
        }))
        .await?;

        Ok(notes)
    }
}
