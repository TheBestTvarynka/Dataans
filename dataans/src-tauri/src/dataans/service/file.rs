use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

use arboard::Clipboard;
use common::note::{File, FileId, FileStatus};
use image::{ImageBuffer, Rgba};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::db::model::File as FileModel;
use crate::dataans::db::Db;
use crate::dataans::DataansError;

// TODO: Introduce `FileServiceError`.

pub struct FileService<D> {
    db: Arc<D>,
    files_path: Arc<Path>,
}

impl<D: Db> FileService<D> {
    pub fn new(db: Arc<D>, files_path: Arc<Path>) -> Self {
        Self { db, files_path }
    }

    pub async fn register_file(&self, file: File) -> Result<(), DataansError> {
        let File {
            id,
            name,
            path,
            status: _,
        } = file;
        let now = OffsetDateTime::now_utc();

        self.db
            .add_file(&FileModel::new(
                id.into(),
                name,
                path.file_name()
                    .ok_or_else(|| {
                        DataansError::IoError(io::Error::new(
                            io::ErrorKind::IsADirectory,
                            format!("invalid file path: {path:?}"),
                        ))
                    })?
                    .to_str()
                    .ok_or_else(|| DataansError::PathIsNotUtf8(path.clone()))?
                    .to_owned(),
                now,
                now,
            ))
            .await?;

        Ok(())
    }

    pub async fn file_by_id(&self, file_id: FileId) -> Result<File, DataansError> {
        let FileModel {
            id,
            name,
            path,
            created_at: _,
            updated_at: _,
            is_deleted: _,
            is_uploaded,
        } = self.db.file_by_id(*file_id.as_ref()).await?;

        let path = self.files_path.join(path);
        let status = FileStatus::status_for_file(&path, is_uploaded);

        Ok(File {
            id: id.into(),
            name,
            path,
            status,
        })
    }

    pub async fn upload_file(&self, id: Uuid, name: String, data: &[u8]) -> Result<File, DataansError> {
        let file_name = format!("{id}_{name}");

        let file_path = self.files_path.join(&file_name);

        fs::write(&file_path, data)?;

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(id, name.clone(), file_name.clone(), now, now))
            .await?;

        let status = FileStatus::status_for_file(&file_path, false);

        Ok(File {
            id: id.into(),
            name,
            path: PathBuf::from(file_name),
            status,
        })
    }

    pub async fn delete_file(&self, file_id: Uuid) -> Result<(), DataansError> {
        let file = self.db.file_by_id(file_id).await?;

        let file_path = self.files_path.join(&file.path);

        fs::remove_file(file_path)?;

        self.db.remove_file(file_id).await?;

        Ok(())
    }

    pub async fn gen_random_avatar(&self) -> Result<File, DataansError> {
        let avatar = avatar_generator::generate::avatar();

        let avatar_id = Uuid::new_v4();
        let avatar_name = format!("{avatar_id}.png");

        let avatar_path = self.files_path.join(&avatar_name);

        avatar
            .save(&avatar_path)
            .map_err(|err| DataansError::ImageGeneration(err.to_string()))?;
        info!("Avatar image path: {:?}", avatar_path);

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(
                avatar_id,
                avatar_name.clone(),
                avatar_name.clone(),
                now,
                now,
            ))
            .await?;

        let status = FileStatus::status_for_file(&avatar_path, false);

        Ok(File {
            id: avatar_id.into(),
            name: avatar_name.clone(),
            path: avatar_name.into(),
            status,
        })
    }

    pub async fn handle_clipboard_image(&self) -> Result<File, DataansError> {
        let mut clipboard = Clipboard::new()?;
        let image_data = clipboard.get_image()?;

        let id = Uuid::new_v4();
        let name = format!("{}.png", Uuid::new_v4());

        let image_path = self.files_path.join(&name);

        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
            image_data.width.try_into().unwrap(),
            image_data.height.try_into().unwrap(),
            image_data.bytes.as_ref(),
        )
        .ok_or_else(|| DataansError::ImageFromRaw)?;
        img.save(&image_path)?;

        let now = OffsetDateTime::now_utc();
        self.db
            .add_file(&FileModel::new(id, name.clone(), name.clone(), now, now))
            .await?;

        let status = FileStatus::status_for_file(&image_path, false);

        Ok(File {
            id: id.into(),
            name: name.clone(),
            path: name.into(),
            status,
        })
    }
}
