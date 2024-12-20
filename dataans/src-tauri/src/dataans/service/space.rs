use std::sync::Arc;

use common::space::OwnedSpace;
use futures::future::try_join_all;

use crate::dataans::db::model::Space as SpaceModel;
use crate::dataans::db::Db;
use crate::dataans::DataansError;

pub struct SpaceService<D> {
    db: Arc<D>,
}

impl<D: Db> SpaceService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn spaces(&self) -> Result<Vec<OwnedSpace>, DataansError> {
        async fn map_model_space_to_space<T: Db>(space: SpaceModel, db: &T) -> Result<OwnedSpace, DataansError> {
            let SpaceModel {
                id,
                name,
                avatar_id,
                created_at,
            } = space;

            let avatar = db.file_by_id(avatar_id).await?;

            Ok(OwnedSpace {
                id: id.into(),
                name: name.into(),
                avatar: avatar.path.into(),
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
