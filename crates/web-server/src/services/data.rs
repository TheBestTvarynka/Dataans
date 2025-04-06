use std::sync::Arc;

use futures::future::try_join_all;
use web_api_types::{BlockId, Note, NoteId, Space, SpaceId, UserId};

use crate::db::{Note as NoteModel, NoteDb, Space as SpaceModel, SpaceDb};
use crate::{Error, Result};

pub struct Data<D> {
    db: Arc<D>,
}

impl<D: NoteDb + SpaceDb> Data<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    async fn check_note_owner(&self, note_id: NoteId, user_id: UserId) -> Result<()> {
        let note_owner_id = self.db.note_owner(note_id.into()).await?;

        if note_owner_id != *user_id.as_ref() {
            Err(Error::InvalidData("note space id"))
        } else {
            Ok(())
        }
    }

    pub async fn add_space(&self, space: Space, user_id: UserId) -> Result<()> {
        if space.user_id != user_id {
            return Err(Error::InvalidData("space user id"));
        }

        let Space {
            id,
            data,
            checksum,
            user_id,
        } = space;

        self.db
            .add_space(&SpaceModel {
                id: id.into(),
                data: data.into(),
                checksum: checksum.into(),
                user_id: user_id.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn update_space(&self, space: Space, user_id: UserId) -> Result<()> {
        if space.user_id != user_id {
            return Err(Error::InvalidData("space user id"));
        }

        let Space {
            id,
            data,
            checksum,
            user_id,
        } = space;

        self.db
            .update_space(&SpaceModel {
                id: id.into(),
                data: data.into(),
                checksum: checksum.into(),
                user_id: user_id.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn remove_space(&self, space_id: SpaceId, user_id: UserId) -> Result<()> {
        let space = self.db.space(space_id.into()).await?;

        if space.user_id != *user_id.as_ref() {
            return Err(Error::InvalidData("space user id"));
        }

        self.db.remove_space(space_id.into()).await?;

        Ok(())
    }

    pub async fn spaces(&self, user_id: UserId) -> Result<Vec<Space>> {
        let spaces = self.db.user_spaces(user_id.into()).await?;

        Ok(spaces
            .into_iter()
            .map(|space| {
                let SpaceModel {
                    id,
                    data,
                    checksum,
                    user_id,
                } = space;

                Space {
                    id: id.into(),
                    data: data.into(),
                    checksum: checksum.into(),
                    user_id: user_id.into(),
                }
            })
            .collect())
    }

    pub async fn add_note(&self, note: Note, user_id: UserId) -> Result<BlockId> {
        let Note {
            id,
            data,
            checksum,
            space_id,
            block_id,
        } = note;

        let space = self.db.space(space_id.into()).await?;
        if space.user_id != *user_id.as_ref() {
            return Err(Error::AccessDenied);
        }

        let block_id = self
            .db
            .add_note(&NoteModel {
                id: id.into(),
                data: data.into(),
                checksum: checksum.into(),
                space_id: space_id.into(),
                block_id: block_id.into(),
            })
            .await?
            .into();

        Ok(block_id)
    }

    pub async fn update_note(&self, note: Note, user_id: UserId) -> Result<()> {
        self.check_note_owner(note.id, user_id).await?;

        let Note {
            id,
            data,
            checksum,
            space_id,
            block_id,
        } = note;

        self.db
            .update_note(&NoteModel {
                id: id.into(),
                data: data.into(),
                checksum: checksum.into(),
                space_id: space_id.into(),
                block_id: block_id.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn remove_note(&self, note_id: NoteId, user_id: UserId) -> Result<()> {
        self.check_note_owner(note_id, user_id).await?;

        self.db.remove_note(note_id.into()).await?;

        Ok(())
    }

    pub async fn notes(&self, note_ids: &[NoteId], user_id: UserId) -> Result<Vec<Note>> {
        let notes = self
            .db
            .notes(
                &try_join_all(note_ids.iter().cloned().map(|note_id| async move {
                    self.check_note_owner(note_id, user_id).await?;

                    Result::Ok(note_id.into())
                }))
                .await?,
            )
            .await?;

        Ok(notes
            .into_iter()
            .map(|note| {
                let NoteModel {
                    id,
                    data,
                    checksum,
                    space_id,
                    block_id,
                } = note;

                Note {
                    id: id.into(),
                    data: data.into(),
                    checksum: checksum.into(),
                    space_id: space_id.into(),
                    block_id: block_id.into(),
                }
            })
            .collect())
    }
}
