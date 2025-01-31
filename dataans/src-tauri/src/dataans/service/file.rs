use std::fs;
use std::path::Path;
use std::sync::Arc;

use arboard::Clipboard;
use common::note::File;
use image::{ImageBuffer, Rgba};
use uuid::Uuid;

use crate::dataans::db::model::File as FileModel;
use crate::dataans::db::{Db, DbError};
use crate::dataans::DataansError;
use crate::{FILES_DIR, IMAGES_DIR};

pub struct FileService<D> {
    db: Arc<D>,
}

impl<D: Db> FileService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn check_default_space_avatar(&self) -> Result<(), DataansError> {
        if let Err(DbError::SqlxError(err)) = self.db.file_by_id(common::DEFAULT_SPACE_AVATAR_ID).await {
            if let sqlx::Error::RowNotFound = err {
                warn!(?err);

                self.db
                    .add_file(&FileModel {
                        id: common::DEFAULT_SPACE_AVATAR_ID,
                        name: "default_space_avatar.png".into(),
                        path: common::DEFAULT_SPACE_AVATAR_PATH.into(),
                        is_synced: false,
                    })
                    .await?;

                Ok(())
            } else {
                Err(DataansError::DbError(DbError::SqlxError(err)))
            }
        } else {
            Ok(())
        }
    }

    pub async fn upload_file(
        &self,
        id: Uuid,
        name: String,
        data: &[u8],
        base_path: &Path,
    ) -> Result<File, DataansError> {
        let file_name = format!("{}_{}", id, name);

        let file_path = base_path.join(FILES_DIR).join(file_name);

        fs::write(&file_path, data)?;

        self.db
            .add_file(&FileModel {
                id,
                name: name.clone(),
                path: file_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(file_path.clone()))?
                    .to_owned(),
                is_synced: false,
            })
            .await?;

        Ok(File {
            id,
            name,
            path: file_path,
        })
    }

    pub async fn delete_file(&self, file_id: Uuid) -> Result<(), DataansError> {
        let file = self.db.file_by_id(file_id).await?;

        fs::remove_file(file.path)?;

        self.db.remove_file(file_id).await?;

        Ok(())
    }

    pub async fn gen_random_avatar(&self, base_path: &Path) -> Result<File, DataansError> {
        let avatar = avatar_generator::generate::avatar();

        let avatar_id = Uuid::new_v4();
        let avatar_name = format!("{}.png", avatar_id);

        let avatar_path = base_path.join(IMAGES_DIR).join(&avatar_name);

        avatar
            .save(&avatar_path)
            .map_err(|err| DataansError::ImageGeneration(err.to_string()))?;
        info!("Avatar image path: {:?}", avatar_path);

        self.db
            .add_file(&FileModel {
                id: avatar_id,
                name: avatar_name.clone(),
                path: avatar_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(avatar_path.clone()))?
                    .to_owned(),
                is_synced: false,
            })
            .await?;

        Ok(File {
            id: avatar_id,
            name: avatar_name,
            path: avatar_path,
        })
    }

    pub async fn handle_clipboard_image(&self, base_path: &Path) -> Result<File, DataansError> {
        let mut clipboard = Clipboard::new()?;
        let image_data = clipboard.get_image()?;

        let id = Uuid::new_v4();
        let name = format!("{}.png", Uuid::new_v4());

        let image_path = base_path.join(IMAGES_DIR).join(&name);

        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
            image_data.width.try_into().unwrap(),
            image_data.height.try_into().unwrap(),
            image_data.bytes.as_ref(),
        )
        .ok_or_else(|| DataansError::ImageFromRaw)?;
        img.save(&image_path)?;

        self.db
            .add_file(&FileModel {
                id,
                name: name.clone(),
                path: image_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(image_path.clone()))?
                    .to_owned(),
                is_synced: false,
            })
            .await?;

        Ok(File {
            id,
            name,
            path: image_path,
        })
    }
}
