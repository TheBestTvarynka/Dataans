use std::sync::Arc;

use web_api_types::{Note, Space, UserId};

use crate::db::{Note as NoteModel, NoteDb, Space as SpaceModel, SpaceDb};
use crate::{Error, Result};

pub struct Data<D> {
    db: Arc<D>,
}

impl<D: NoteDb + SpaceDb> Data<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
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

    pub async fn add_note(&self, note: Note, user_id: UserId) -> Result<()> {
        let space = self.db.space(*note.space_id.as_ref()).await?;

        if space.user_id != *user_id.as_ref() {
            return Err(Error::InvalidData("note space id"));
        }

        let Note {
            id,
            data,
            checksum,
            space_id,
            block_id,
        } = note;

        self.db
            .add_note(&NoteModel {
                id: id.into(),
                data: data.into(),
                checksum: checksum.into(),
                space_id: space_id.into(),
                block_id: block_id.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn update_note(&self, note: Note, user_id: UserId) -> Result<()> {
        let space = self.db.space(*note.space_id.as_ref()).await?;

        if space.user_id != *user_id.as_ref() {
            return Err(Error::InvalidData("note space id"));
        }

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
}
