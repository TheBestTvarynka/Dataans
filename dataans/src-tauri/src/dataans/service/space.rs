use std::sync::Arc;

use common::space::{Avatar, CreateSpaceOwned, DeleteSpace, Id as SpaceId, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;
use time::OffsetDateTime;

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

    pub async fn create_space(&self, space: CreateSpaceOwned) -> Result<OwnedSpace, DataansError> {
        let CreateSpaceOwned { id, name, avatar } = space;

        let created_at = OffsetDateTime::now_utc();

        self.db
            .create_space(&SpaceModel {
                id: id.inner(),
                name: name.clone().into(),
                avatar_id: avatar.id(),
                created_at,
                updated_at: created_at,
                is_synced: false,
            })
            .await?;

        Ok(OwnedSpace {
            id,
            name,
            avatar,
            created_at: created_at.into(),
            updated_at: created_at.into(),
            is_synced: false.into(),
        })
    }

    pub async fn update_space(&self, space_data: UpdateSpace<'static>) -> Result<OwnedSpace, DataansError> {
        let UpdateSpace {
            id: space_id,
            name,
            avatar,
        } = space_data;

        let SpaceModel {
            id,
            name: _,
            avatar_id: _,
            created_at,
            updated_at: _,
            is_synced: _,
        } = self.db.space_by_id(space_id.inner()).await?;

        let updated_at = OffsetDateTime::now_utc();

        self.db
            .update_space(&SpaceModel {
                id,
                name: name.clone().into(),
                avatar_id: avatar.id(),
                created_at,
                updated_at,
                is_synced: false,
            })
            .await?;

        Ok(OwnedSpace {
            id: space_id,
            name,
            avatar,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            is_synced: false.into(),
        })
    }

    pub async fn delete_space(&self, id: DeleteSpace) -> Result<(), DataansError> {
        let DeleteSpace { id } = id;

        Ok(self.db.remove_space(id.inner()).await?)
    }

    async fn map_model_space_to_space<T: Db>(space: SpaceModel, db: &T) -> Result<OwnedSpace, DataansError> {
        let SpaceModel {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            is_synced,
        } = space;

        let FileModel {
            id: avatar_id,
            name: _,
            path: avatar_path,
            is_synced: _,
        } = db.file_by_id(avatar_id).await?;

        Ok(OwnedSpace {
            id: id.into(),
            name: name.into(),
            avatar: Avatar::new(avatar_id, avatar_path),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            is_synced: is_synced.into(),
        })
    }

    pub async fn spaces(&self) -> Result<Vec<OwnedSpace>, DataansError> {
        let spaces = try_join_all(
            self.db
                .spaces()
                .await?
                .into_iter()
                .map(|space| Self::map_model_space_to_space(space, &*self.db)),
        )
        .await?;

        Ok(spaces)
    }

    pub async fn space_by_id(&self, space_id: SpaceId) -> Result<OwnedSpace, DataansError> {
        Self::map_model_space_to_space(self.db.space_by_id(space_id.inner()).await?, &*self.db).await
    }
}
