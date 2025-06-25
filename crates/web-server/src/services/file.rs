use std::marker::Unpin;
use std::path::PathBuf;

use rocket::tokio::fs::File;
use rocket::tokio::io::{copy, AsyncRead};
use uuid::Uuid;

use crate::Result;

pub trait FileSaver: Send + Sync {
    async fn save_file(&self, id: Uuid, reader: impl AsyncRead + Unpin) -> Result<()>;
    async fn open_file(&self, id: Uuid) -> Result<(Option<usize>, impl AsyncRead + Send)>;
}

#[derive(Default, Debug)]
pub struct Fs {
    dest: PathBuf,
}

impl Fs {
    pub fn new(dest: PathBuf) -> Self {
        Self { dest }
    }
}

impl FileSaver for Fs {
    #[instrument(ret, skip(reader))]
    async fn save_file(&self, id: Uuid, mut reader: impl AsyncRead + Unpin) -> Result<()> {
        let mut file = File::create(self.dest.join(id.to_string())).await?;

        copy(&mut reader, &mut file).await?;

        Ok(())
    }

    #[instrument(err)]
    async fn open_file(&self, id: Uuid) -> Result<(Option<usize>, impl AsyncRead + Send)> {
        // TODO: use buf reader.
        let data = File::open(self.dest.join(id.to_string())).await?;
        let size = data
            .metadata()
            .await
            .ok()
            .map(|metadata| usize::try_from(metadata.len()).expect("file size should fit in usize"));

        Ok((size, data))
    }
}
