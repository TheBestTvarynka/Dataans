use std::marker::Unpin;

use rocket::tokio::io::AsyncRead;
use uuid::Uuid;

use crate::Result;

pub trait FileSaver: Send + Sync {
    async fn save_file(&self, id: Uuid, reader: impl AsyncRead + Unpin) -> Result<()>;
    async fn open_file(&self, id: Uuid) -> Result<(Option<usize>, impl AsyncRead + Send)>;
}

#[cfg(feature = "fs")]
mod fs {
    use std::path::PathBuf;

    use rocket::tokio::fs::File;
    use rocket::tokio::io::{copy, AsyncRead};
    use uuid::Uuid;

    use crate::services::FileSaver;
    use crate::Result;

    #[derive(Debug)]
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
}

#[cfg(feature = "fs")]
pub use fs::Fs;

#[cfg(feature = "tigris")]
mod tigris {
    use std::fmt;

    use aws_config::SdkConfig;
    use aws_sdk_s3::Client;
    use rocket::tokio::io::{AsyncRead, AsyncReadExt};
    use uuid::Uuid;

    use crate::services::FileSaver;
    use crate::{Error, Result};

    pub struct Tigris {
        client: Client,
        bucket: String,
    }

    impl fmt::Debug for Tigris {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Tigris")
                .field("client", &"<...>")
                .field("bucket", &self.bucket)
                .finish()
        }
    }

    impl Tigris {
        pub fn new(config: SdkConfig, bucket: String) -> Self {
            Self {
                client: Client::new(&config),
                bucket,
            }
        }
    }

    impl FileSaver for Tigris {
        #[instrument(ret, skip(reader))]
        async fn save_file(&self, id: Uuid, mut reader: impl AsyncRead + Unpin) -> Result<()> {
            let mut data = Vec::new();
            reader.read_to_end(&mut data).await?;

            let object = self
                .client
                .put_object()
                .bucket(&self.bucket)
                .key(id.to_string())
                // I hate the `aws_sdk_s3` body API.
                // https://github.com/awslabs/aws-sdk-rust/discussions/361
                .body(data.into())
                .send()
                .await
                .map_err(|err| {
                    error!(?err, "Failed to save file to S3");
                    Error::FileSaver(err.to_string())
                })?;

            trace!(?object, "File has been saved to S3");

            Ok(())
        }

        #[instrument(err)]
        async fn open_file(&self, id: Uuid) -> Result<(Option<usize>, impl AsyncRead + Send)> {
            let object = self
                .client
                .get_object()
                .bucket(&self.bucket)
                .key(id.to_string())
                .send()
                .await
                .map_err(|err| {
                    error!(?err, "Failed to open file from S3");
                    Error::FileSaver(err.to_string())
                })?;
            let data = object.body.into_async_read();
            let size = object
                .content_length
                .map(|len| usize::try_from(len).expect("file size should fit in"));

            Ok((size, data))
        }
    }
}

#[cfg(feature = "tigris")]
pub use tigris::Tigris;
