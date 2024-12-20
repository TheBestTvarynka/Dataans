use std::sync::Arc;

use common::space::{Avatar, DeleteSpace, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;

use crate::dataans::db::model::{File as FileModel, Space as SpaceModel};
use crate::dataans::db::Db;
use crate::dataans::DataansError;

pub struct SpaceService<D> {
    db: Arc<D>,
}

impl<D: Db> SpaceService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn create_space(&self, space: OwnedSpace) -> Result<(), DataansError> {
        let OwnedSpace {
            id,
            name,
            avatar,
            created_at,
        } = space;

        self.db
            .create_space(&SpaceModel {
                id: id.inner(),
                name: name.into(),
                avatar_id: avatar.id(),
                created_at: created_at.into(),
            })
            .await?;

        Ok(())
    }

    pub async fn update_space(&self, space_data: UpdateSpace<'static>) -> Result<(), DataansError> {
        let UpdateSpace { id, name, avatar } = space_data;

        let SpaceModel {
            id,
            name: _,
            avatar_id: _,
            created_at,
        } = self.db.space_by_id(id.inner()).await?;

        Ok(self
            .db
            .update_space(&SpaceModel {
                id,
                name: name.into(),
                avatar_id: avatar.id(),
                created_at,
            })
            .await?)
    }

    pub async fn delete_space(&self, id: DeleteSpace) -> Result<(), DataansError> {
        let DeleteSpace { id } = id;

        Ok(self.db.remove_space(id.inner()).await?)
    }

    pub async fn spaces(&self) -> Result<Vec<OwnedSpace>, DataansError> {
        async fn map_model_space_to_space<T: Db>(space: SpaceModel, db: &T) -> Result<OwnedSpace, DataansError> {
            let SpaceModel {
                id,
                name,
                avatar_id,
                created_at,
            } = space;

            let FileModel {
                id: avatar_id,
                name: _,
                path: avatar_path,
            } = db.file_by_id(avatar_id).await?;

            Ok(OwnedSpace {
                id: id.into(),
                name: name.into(),
                avatar: Avatar::new(avatar_id, avatar_path),
                created_at: created_at.into(),
            })
        }

        let spaces = try_join_all(
            self.db
                .spaces()
                .await?
                .into_iter()
                .map(|space| map_model_space_to_space(space, &*self.db)),
        )
        .await?;

        Ok(spaces)
    }
}
