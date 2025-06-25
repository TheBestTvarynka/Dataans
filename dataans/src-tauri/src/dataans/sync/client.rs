use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use sha2::Sha256;
use url::Url;
use uuid::Uuid;
use web_api_types::{AuthToken, Blocks, Operation, UserId, AUTH_HEADER_NAME};

use super::SyncError;
use crate::dataans::crypto::{decrypt, encrypt, EncryptionKey};
use crate::dataans::db::{OperationRecord, OperationRecordOwned};
use crate::dataans::sync::hash::Hash;

pub struct Client {
    client: reqwest::Client,
    sync_server: Url,
    encryption_key: EncryptionKey,
}

impl Client {
    pub fn new(sync_server: Url, auth_token: AuthToken, encryption_key: EncryptionKey) -> Result<Self, SyncError> {
        let client = ClientBuilder::new()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(AUTH_HEADER_NAME, HeaderValue::from_str(auth_token.as_ref())?);
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

    #[instrument(ret, skip(self))]
    pub async fn blocks(&self, items_per_block: usize) -> Result<Vec<Vec<u8>>, SyncError> {
        let mut blocks_url = self.sync_server.join("data/block")?;
        blocks_url
            .query_pairs_mut()
            .append_pair("items_per_block", &items_per_block.to_string());

        let blocks = self.client.get(blocks_url).send().await?.json::<Blocks>().await?;

        let blocks = blocks.0.into_iter().map(|block| block.0).collect::<Vec<_>>();

        Ok(blocks)
    }

    #[instrument(ret, skip(self))]
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

    #[instrument(ret, skip(self, operations))]
    pub async fn upload_operations(&self, operations: &[&OperationRecord<'_>]) -> Result<(), SyncError> {
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

    #[instrument(ret, skip(self, data))]
    pub async fn upload_file(&self, id: Uuid, data: &[u8]) -> Result<(), SyncError> {
        self.client
            .post(self.sync_server.join("file/")?.join(&id.to_string())?)
            .body(data.to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    #[instrument(err, skip(self))]
    pub async fn download_file(&self, id: Uuid) -> Result<Vec<u8>, SyncError> {
        let response = self
            .client
            .get(self.sync_server.join("file/")?.join(&id.to_string())?)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.bytes().await?.to_vec())
    }
}
