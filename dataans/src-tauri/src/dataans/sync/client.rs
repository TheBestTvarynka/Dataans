use std::path::Path;
use std::time::Duration;

use common::profile::AuthorizationToken;
use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use url::Url;
use uuid::Uuid;
use web_api_types::{Blocks, Operation};

use super::SyncError;
use crate::dataans::crypto::{EncryptionKey, decrypt, decrypt_data, encrypt, encrypt_data};
use crate::dataans::db::{OperationRecord, OperationRecordOwned};
use crate::dataans::sync::hash::Hash;

/// Sync/Back up server API client.
///
/// This client is used for communication with the sync (back up) server.
pub struct Client {
    client: reqwest::Client,
    sync_server: Url,
    encryption_key: EncryptionKey,
}

impl Client {
    /// Creates a new client.
    pub fn new(
        sync_server: Url,
        encryption_key: EncryptionKey,
        auth_token: &AuthorizationToken,
    ) -> Result<Self, SyncError> {
        let client = ClientBuilder::new()
            .default_headers({
                let mut headers = HeaderMap::new();

                headers.insert(
                    "Cookie",
                    HeaderValue::from_str(&format!("CF_Authorization={}", auth_token.as_ref()))?,
                );

                headers
            })
            .http2_keep_alive_interval(Some(Duration::from_secs(30)))
            .http2_keep_alive_timeout(Duration::from_secs(30))
            .http2_keep_alive_while_idle(true)
            .build()?;

        Ok(Self {
            client,
            sync_server,
            encryption_key,
        })
    }

    /// Sends a request to the protected endpoint.
    ///
    /// This method is used for checking whether the auth token is correct.
    pub async fn auth_health(&self) -> Result<(), SyncError> {
        let health_url = self.sync_server.join("health/auth")?;
        trace!(?health_url, "Auth health check URL");

        let response = self.client.get(health_url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(SyncError::HealthCheckFailed(response.status()))
        }
    }

    /// Requests server's blocks hashes.
    #[instrument(err, skip(self))]
    pub async fn blocks(&self, items_per_block: usize) -> Result<Vec<Vec<u8>>, SyncError> {
        let mut blocks_url = self.sync_server.join("data/block")?;
        blocks_url
            .query_pairs_mut()
            .append_pair("items_per_block", &items_per_block.to_string());

        let blocks = self.client.get(blocks_url).send().await?.json::<Blocks>().await?;

        let blocks = blocks.0.into_iter().map(|block| block.0).collect::<Vec<_>>();

        Ok(blocks)
    }

    /// Requests operation stored on the server.
    ///
    /// The server will skip the first `operations_to_skip` operations and will return the rest of them.
    /// This method automatically decrypt the received operation.
    #[instrument(err, skip(self))]
    pub async fn operations(&self, operations_to_skip: usize) -> Result<Vec<OperationRecordOwned>, SyncError> {
        let mut operations_url = self.sync_server.join("data/operation")?;
        operations_url
            .query_pairs_mut()
            .append_pair("operations_to_skip", &operations_to_skip.to_string());

        let operations = self
            .client
            .get(operations_url)
            .send()
            .await?
            .json::<Vec<Operation>>()
            .await?;

        let operations = operations
            .into_iter()
            .map(|operation| {
                Result::<OperationRecordOwned, SyncError>::Ok(decrypt(operation.data.as_ref(), &self.encryption_key)?)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(operations)
    }

    /// Sends the provided operations to the server.
    ///
    /// This method automatically encrypts provided operations.
    #[instrument(err, skip(self, operations))]
    pub async fn upload_operations(&self, operations: &[OperationRecord<'_>]) -> Result<(), SyncError> {
        let operations = operations
            .iter()
            .map(|operation| {
                let encrypted_data = encrypt(operation, &self.encryption_key)?;
                Ok(Operation {
                    id: operation.id.into(),
                    created_at: operation.created_at.into(),
                    data: encrypted_data.into(),
                    checksum: operation.digest::<Sha256>().to_vec().into(),
                })
            })
            .collect::<Result<Vec<_>, SyncError>>()?;

        self.client
            .post(self.sync_server.join("data/operation")?)
            .json(&operations)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Uploads the file to the server.
    ///
    /// The provided path must be absolute in the file system.
    #[instrument(err, skip(self))]
    pub async fn upload_file(&self, id: Uuid, path: &Path) -> Result<(), SyncError> {
        let file_data = tokio::fs::read(path).await?;

        let data = encrypt_data(&file_data, &self.encryption_key)?;

        self.client
            .post(self.sync_server.join("file/")?.join(&id.to_string())?)
            .body(data)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Checks whether the file with the given ID exists on the server.
    pub async fn exists(&self, id: Uuid) -> Result<bool, SyncError> {
        let response = self
            .client
            .get(self.sync_server.join("file/")?.join(&id.to_string())?)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<bool>().await?)
    }

    /// Downloads the file from the server.
    ///
    /// The provided path must be absolute in the file system.
    #[instrument(err, skip(self))]
    pub async fn download_file(&self, id: Uuid, path: &Path) -> Result<(), SyncError> {
        let response = self
            .client
            .get(self.sync_server.join("file/")?.join(&id.to_string())?)
            .send()
            .await?
            .error_for_status()?;

        let data = response.bytes().await?.to_vec();
        let file_data = decrypt_data(&data, &self.encryption_key)?;

        tokio::fs::write(path, &file_data).await?;

        Ok(())
    }
}
