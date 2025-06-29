use std::fs;
use std::path::Path;
use std::sync::Arc;

use arboard::Clipboard;
use common::note::{File, FileStatus};
use image::{ImageBuffer, Rgba};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::db::model::File as FileModel;
use crate::dataans::db::Db;
use crate::dataans::DataansError;
use crate::{FILES_DIR, IMAGES_DIR};

pub struct FileService<D> {
    db: Arc<D>,
}

impl<D: Db> FileService<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn upload_file(
        &self,
        id: Uuid,
        name: String,
        data: &[u8],
        base_path: &Path,
    ) -> Result<File, DataansError> {
        let file_name = format!("{id}_{name}");

        let file_path = base_path.join(FILES_DIR).join(file_name);

        fs::write(&file_path, data)?;

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(
                id,
                name.clone(),
                file_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(file_path.clone()))?
                    .to_owned(),
                now,
                now,
            ))
            .await?;

        let status = FileStatus::status_for_file(&file_path, false);

        Ok(File {
            id: id.into(),
            name,
            path: file_path,
            status,
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
        let avatar_name = format!("{avatar_id}.png");

        let avatar_path = base_path.join(IMAGES_DIR).join(&avatar_name);

        avatar
            .save(&avatar_path)
            .map_err(|err| DataansError::ImageGeneration(err.to_string()))?;
        info!("Avatar image path: {:?}", avatar_path);

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(
                avatar_id,
                avatar_name.clone(),
                avatar_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(avatar_path.clone()))?
                    .to_owned(),
                now,
                now,
            ))
            .await?;

        let status = FileStatus::status_for_file(&avatar_path, false);

        Ok(File {
            id: avatar_id.into(),
            name: avatar_name,
            path: avatar_path,
            status,
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

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(
                id,
                name.clone(),
                image_path
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(image_path.clone()))?
                    .to_owned(),
                now,
                now,
            ))
            .await?;

        let status = FileStatus::status_for_file(&image_path, false);

        Ok(File {
            id: id.into(),
            name,
            path: image_path,
            status,
        })
    }
}
