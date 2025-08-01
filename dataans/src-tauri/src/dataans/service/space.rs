use std::sync::Arc;

use common::error::CommandError;
use common::space::{Avatar, CreateSpaceOwned, DeleteSpace, Id as SpaceId, OwnedSpace, UpdateSpace};
use futures::future::try_join_all;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::DataansError;
use crate::dataans::db::model::{File as FileModel, Space as SpaceModel};
use crate::dataans::db::{Db, DbError};

#[derive(Debug, Error)]
pub enum SpaceServiceError {
    #[error(transparent)]
    DbError(DbError),

    #[error("not found")]
    NotFound,
}

impl From<DbError> for SpaceServiceError {
    fn from(err: DbError) -> Self {
        if let DbError::SqlxError(sqlx::Error::RowNotFound) = err {
            Self::NotFound
        } else {
            Self::DbError(err)
        }
    }
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

        let avatar_id = if avatar.id() == common::DEFAULT_SPACE_AVATAR_ID {
            // If the user decided to use the default avatar, we should create a new avatar file with a default image path in the database.
            let avatar_id = Uuid::new_v4();
            let avatar_name = format!("{avatar_id}.png");

            self.db
                .add_file(&FileModel::new(
                    avatar_id,
                    avatar_name,
                    avatar.path().to_owned(),
                    created_at,
                    created_at,
                ))
                .await?;

            avatar_id
        } else {
            avatar.id()
        };

        self.db
            .create_space(&SpaceModel::new(
                id.inner(),
                name.clone().into(),
                avatar_id,
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
            created_at: _,
            updated_at: _,
            is_deleted: _,
            is_uploaded: _,
        } = db.file_by_id(avatar_id).await?;

        Ok(OwnedSpace {
            id: id.into(),
            name: name.into(),
            avatar: Avatar::new(avatar_id.into(), avatar_path),
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
