use std::sync::Arc;

use common::error::CommandError;
use common::space::{Avatar, CreateSpaceOwned, DeleteSpace, Id as SpaceId, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;
use thiserror::Error;
use time::OffsetDateTime;

use crate::dataans::db::model::{File as FileModel, Space as SpaceModel};
use crate::dataans::db::{Db, DbError};
use crate::dataans::DataansError;

#[derive(Debug, Error)]
pub enum SpaceServiceError {
    #[error(transparent)]
    DbError(#[from] DbError),
}

impl From<SpaceServiceError> for CommandError {
    fn from(error: SpaceServiceError) -> Self {
        DataansError::SpaceService(error).into()
    }
}

type SpaceServiceResult<T> = Result<T, SpaceServiceError>;

pub struct SpaceService<D> {
    db: Arc<D>,
}

impl<D: Db> SpaceService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn create_space(&self, space: CreateSpaceOwned) -> SpaceServiceResult<OwnedSpace> {
        let CreateSpaceOwned { id, name, avatar } = space;

        let created_at = OffsetDateTime::now_utc();

        self.db
            .create_space(&SpaceModel::new(
                id.inner(),
                name.clone().into(),
                avatar.id(),
                created_at,
                created_at,
            ))
            .await?;

        Ok(OwnedSpace {
            id,
            name,
            avatar,
            created_at: created_at.into(),
            updated_at: created_at.into(),
        })
    }

    pub async fn update_space(&self, space_data: UpdateSpace<'static>) -> SpaceServiceResult<OwnedSpace> {
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
            is_deleted: _,
        } = self.db.space_by_id(space_id.inner()).await?;

        let updated_at = OffsetDateTime::now_utc();

        self.db
            .update_space(&SpaceModel::new(
                id,
                name.clone().into(),
                avatar.id(),
                created_at,
                updated_at,
            ))
            .await?;

        Ok(OwnedSpace {
            id: space_id,
            name,
            avatar,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        })
    }

    pub async fn delete_space(&self, id: DeleteSpace) -> SpaceServiceResult<()> {
        let DeleteSpace { id } = id;

        Ok(self.db.remove_space(id.inner()).await?)
    }

    pub async fn map_model_space_to_space(space: SpaceModel, db: &D) -> SpaceServiceResult<OwnedSpace> {
        let SpaceModel {
            id,
            name,
            avatar_id,
            created_at,
            updated_at,
            is_deleted: _,
        } = space;

        let FileModel {
            id: avatar_id,
            name: _,
            path: avatar_path,
            is_deleted: _,
        } = db.file_by_id(avatar_id).await?;

        Ok(OwnedSpace {
            id: id.into(),
            name: name.into(),
            avatar: Avatar::new(avatar_id, avatar_path),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        })
    }

    pub async fn spaces(&self) -> SpaceServiceResult<Vec<OwnedSpace>> {
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

    pub async fn space_by_id(&self, space_id: SpaceId) -> SpaceServiceResult<OwnedSpace> {
        Self::map_model_space_to_space(self.db.space_by_id(space_id.inner()).await?, &*self.db).await
    }
}
