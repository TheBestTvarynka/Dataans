use std::path::Path;
use std::time::Duration;

use base64ct::{Base64, Encoding};
use common::profile::AuthorizationToken;
use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;
use web_api_types::{Blocks, Operation, User};

use super::SyncError;
use crate::dataans::crypto::{EncryptionKey, decrypt, decrypt_data, encrypt, encrypt_data};
use crate::dataans::db::{OperationRecord, OperationRecordOwned};
use crate::dataans::sync::hash::Hash;

macro_rules! check_token_expiration {
    ($expired_at:expr) => {
        let now = time::OffsetDateTime::now_utc();
        if now >= $expired_at {
            return Err(super::SyncError::TokenExpired);
        }
    };
}

/// Sync/Back up server API client.
///
/// This client is used for communication with the sync (back up) server.
pub struct Client {
    client: reqwest::Client,
    sync_server: Url,
    encryption_key: EncryptionKey,
    /// Authorization token expiration time.
    expires_at: OffsetDateTime,
}

impl Client {
    /// Creates a new client.
    pub fn new(
        sync_server: Url,
        encryption_key: EncryptionKey,
        auth_token: &AuthorizationToken,
    ) -> Result<Self, SyncError> {
        let expires_at = extract_expiration_time(auth_token)?;

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
            expires_at,
        })
    }

    /// Sends a request to the protected endpoint.
    ///
    /// This method is used for checking whether the auth token is correct.
    pub async fn auth_health(&self) -> Result<(), SyncError> {
        check_token_expiration!(self.expires_at);

        let response = self.client.get(self.sync_server.join("health/auth")?).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(SyncError::HealthCheckFailed(response.status()))
        }
    }

    /// Requests server's blocks hashes.
    #[instrument(err, skip(self))]
    pub async fn blocks(&self, items_per_block: usize) -> Result<Vec<Vec<u8>>, SyncError> {
        check_token_expiration!(self.expires_at);

        let mut blocks_url = self.sync_server.join("data/block")?;
        blocks_url
            .query_pairs_mut()
            .append_pair("items_per_block", &items_per_block.to_string());

        let blocks = self
            .client
            .get(blocks_url)
            .send()
            .await?
            .error_for_status()?
            .json::<Blocks>()
            .await?;

        let blocks = blocks.0.into_iter().map(|block| block.0).collect::<Vec<_>>();

        Ok(blocks)
    }

    /// Requests operation stored on the server.
    ///
    /// The server will skip the first `operations_to_skip` operations and will return the rest of them.
    /// This method automatically decrypt the received operation.
    #[instrument(err, skip(self))]
    pub async fn operations(&self, operations_to_skip: usize) -> Result<Vec<OperationRecordOwned>, SyncError> {
        check_token_expiration!(self.expires_at);

        let mut operations_url = self.sync_server.join("data/operation")?;
        operations_url
            .query_pairs_mut()
            .append_pair("operations_to_skip", &operations_to_skip.to_string());

        let operations = self
            .client
            .get(operations_url)
            .send()
            .await?
            .error_for_status()?
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
        check_token_expiration!(self.expires_at);

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

        let _ = self
            .client
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
        check_token_expiration!(self.expires_at);

        let file_data = tokio::fs::read(path).await?;

        let data = encrypt_data(&file_data, &self.encryption_key)?;

        let _ = self
            .client
            .post(self.sync_server.join("file/")?.join(&id.to_string())?)
            .body(data)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Checks whether the file with the given ID exists on the server.
    pub async fn exists(&self, id: Uuid) -> Result<bool, SyncError> {
        check_token_expiration!(self.expires_at);

        let response = self
            .client
            .get(
                self.sync_server
                    .join("file/")?
                    .join(&format!("{id}/"))?
                    .join("exists")?,
            )
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
        check_token_expiration!(self.expires_at);

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

    /// Returns the [User] object from the sync server.
    ///
    /// There is noting special about the [User] object. It is only used to validate the user's
    /// password and salt by checking the secret key hash.
    #[instrument(err, skip(self))]
    pub async fn user(&self) -> Result<User, SyncError> {
        check_token_expiration!(self.expires_at);

        let response = self
            .client
            .get(self.sync_server.join("user/")?)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<User>().await?)
    }

    /// Initializes the user on the sync server.
    ///
    /// This method **must** be called only _once_.
    #[instrument(err, skip(self))]
    pub async fn init_user(&self, user: &User) -> Result<(), SyncError> {
        check_token_expiration!(self.expires_at);

        let _ = self
            .client
            .post(self.sync_server.join("user/")?)
            .json(user)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

fn extract_expiration_time(auth_token: &AuthorizationToken) -> Result<OffsetDateTime, SyncError> {
    let mut token_parts = auth_token.as_ref().split('.');

    // Skip header.
    token_parts.next();

    let body = token_parts
        .next()
        .ok_or_else(|| SyncError::InvalidAuthToken("JWT body is missing".into()))?;

    let payload = Base64::decode_vec(body)
        .map_err(|err| SyncError::InvalidAuthToken(format!("base64: failed to decode JWT body: {err}")))?;

    let payload_json: serde_json::Value = serde_json::from_slice(&payload)
        .map_err(|err| SyncError::InvalidAuthToken(format!("JSON: failed to parse JWT body: {err}")))?;

    if let Some(exp) = payload_json.get("exp").and_then(|v| v.as_i64()) {
        Ok(OffsetDateTime::from_unix_timestamp(exp)
            .map_err(|err| SyncError::InvalidAuthToken(format!("invalid JWT body 'exp' value: {err}")))?)
    } else {
        Err(SyncError::InvalidAuthToken(
            "JWT body 'exp' field is missing or invalid".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationToken, extract_expiration_time};

    #[test]
    fn extract_jwt_expiration_time() {
        let auth_token = AuthorizationToken::from(String::from(
            // Invalid auth token. It is made based on a real token using https://crypto.qkation.com/jwt but with altered fields and signed with a dummy key.
            "eyJhbGciOiJSUzI1NiIsImtpZCI6IjkzNDFkYzI2OGJlNjlkYTg4MmM5YjlkZDFjOThkMWE1YmIyYWY3ZDdiYzk3YjgxZGE0OTgzMTdmYjliIn0.eyJhdWQiOlsiZmNmMTlkN2Y3N2E5N2M5MzI5NTYwYzcwOWY2ZWRkOWQzYWZhZWE1MTAzYWExYThlZWJkNzgwN2YwOTYxNjliOCJdLCJlbWFpbCI6ImZlbmlvZXZ3bmx2amtmdmp3ZWZuZWx2d2VrQHRlc3QuY29tIiwiZXhwIjoxMzEyODI2MzA1LCJpYXQiOjE3NjAyMTgzMDUsIm5iZiI6MTc2MDIxODMwNSwiaXNzIjoiaHR0cHM6Ly9mb3JlaWZvaWVyZmppb3JlLmNsb3VkZmxhcmVhY2Nlc3MuY29tIiwidHlwZSI6ImFwcCIsImlkZW50aXR5X25vbmNlIjoiMHhCQ1NxN3BNbzQyTFBrRiIsInN1YiI6ImQzZWEzZmJhLWFjMzgtNTY0ZS1iODQxLWE0NjVkOWFkNjk2NyIsImNvdW50cnkiOiJVQSJ9.TBlPMh7kpT2XdBGQfYV8Ullm5x6OYzaA7IxPAWNyDBsl+wL7bWp8F8qsAja2iKhzdC8aLe9u/D71o8E6J9XhcqZ/37fCyHfAHHuDAUXhIdMyJ7RTdgy4mNq8U0HkAzByGI+1ziZoUAOZrILA4gXe/HsDOeoSW+i2CJBCs+UDsp8lggUjAMvSwRb62If09vNaq8mkn5bhcJ+rCAHgeHOfbuEwhFpaMxWlCBIJTkReRMYkaY8zLoPCRjZuNCVHMlOajK3Gs16kRw+jRSBBuz2Eu4+izBbewAxenV0Yz1FLg/IJx4SQPuNH9OzRAeXNLh4BEysCup9HxRbiqhsZf9QSmw",
        ));
        println!("{:?}", extract_expiration_time(&auth_token).unwrap());
    }
}
