use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;
use web_api_types::{AuthToken, UserId, AUTH_HEADER_NAME, Blocks, Operation};

use super::SyncError;

pub struct Client {
    client: reqwest::Client,
    sync_server: Url,
}

impl Client {
    pub fn new(sync_server: Url, auth_token: AuthToken) -> Result<Self, SyncError> {
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

        Ok(Self { client, sync_server })
    }

    pub async fn blocks(&self, items_per_block: usize) -> Result<Vec<Vec<u8>>, SyncError> {
        let mut blocks_url = self.sync_server.join("block")?;
        blocks_url
            .query_pairs_mut()
            .append_pair("items_per_block", &items_per_block.to_string());

        let blocks = self.client.get(blocks_url).send().await?.json::<Blocks>().await?;

        let blocks = blocks.0.into_iter().map(|block| block.0).collect::<Vec<_>>();

        Ok(blocks)
    }

    pub async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>, SyncError> {
        let mut operations_url = self.sync_server.join("operation")?;
        operations_url
            .query_pairs_mut()
            .append_pair("operations_to_skip", &operations_to_skip.to_string());

        let operations = self.client.get(operations_url).send().await?.json::<Vec<Operation>>().await?;

        Ok(operations)
    }

    pub async fn add_operations(&self, operations: Vec<Operation>) -> Result<(), SyncError> {
        self.client
            .post(self.sync_server.join("operation")?)
            .json(&operations)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
