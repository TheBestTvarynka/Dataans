use aes_gcm::aead::Aead;
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, KeySizeUser, Nonce};
use pbkdf2::pbkdf2_hmac;
use rand::rngs::OsRng;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::digest::typenum::Unsigned;
use sha2::{Digest, Sha256};
use thiserror::Error;

const NONCE_LENGTH: usize = 12;
const HMAC_SHA256_CHECKSUM_LENGTH: usize = 32;

pub type EncryptionKey = Key<Aes256Gcm>;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid encryption key or IV length")]
    InvalidKeyLength,

    #[error("encryption error: {0}")]
    Encryption(#[from] aes_gcm::Error),

    #[error("failed to decrypt the data: {0}")]
    DecryptionFailed(&'static str),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

type CryptoResult<T> = Result<T, CryptoError>;

pub fn encrypt_data(data: &[u8], key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    // Encryption
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);

    let cipher_text = cipher.encrypt(&nonce, data)?;

    let mut result = nonce.as_slice().to_vec();
    result.extend_from_slice(&cipher_text);

    // Checksum
    let checksum = {
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|err| {
            error!(?err, "Failed to initialize HMAC-SHA256");
            CryptoError::InvalidKeyLength
        })?;

        mac.update(&nonce);
        mac.update(data);

        mac.finalize().into_bytes()
    };
    result.extend_from_slice(&checksum);

    // result = nonce + cipher_text + checksum
    Ok(result)
}

pub fn decrypt_data(data: &[u8], key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    // data = nonce + cipher_text + checksum

    if data.len() < NONCE_LENGTH + HMAC_SHA256_CHECKSUM_LENGTH {
        return Err(CryptoError::DecryptionFailed("invalid data length"));
    }

    let (nonce, data) = data.split_at(NONCE_LENGTH);
    let (cipher_text, checksum) = data.split_at(data.len() - HMAC_SHA256_CHECKSUM_LENGTH);

    let nonce = Nonce::from_slice(nonce);

    // Decryption
    let cipher = Aes256Gcm::new(key);

    let decrypted = cipher.decrypt(nonce, cipher_text)?;

    // Checksum verification
    let expected_checksum = {
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|err| {
            error!(?err, "Failed to initialize HMAC-SHA256");
            CryptoError::InvalidKeyLength
        })?;

        mac.update(nonce);
        mac.update(&decrypted);

        mac.finalize().into_bytes().to_vec()
    };

    if expected_checksum != checksum {
        return Err(CryptoError::DecryptionFailed("message altered"));
    }

    Ok(decrypted)
}

pub fn encrypt<T: Serialize>(data: &T, key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    let data = serde_json::to_vec(data)?;

    encrypt_data(&data, key)
}

pub fn decrypt<T: DeserializeOwned>(data: &[u8], key: &EncryptionKey) -> CryptoResult<T> {
    let data = decrypt_data(data, key)?;

    Ok(serde_json::from_slice(&data)?)
}

pub fn derive_encryption_key(password: &[u8], salt: &[u8]) -> CryptoResult<EncryptionKey> {
    let password = Sha256::digest(password).to_vec();
    let salt = Sha256::digest(salt).to_vec();

    let mut key = [0_u8; <Aes256Gcm as KeySizeUser>::KeySize::USIZE];
    pbkdf2_hmac::<Sha256>(&password, &salt, 1_200_000, &mut key);

    Ok(key.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn data_encryption() {
        let key = b"oeifvncpfiejnvdjpvnwifvj12345678";
        let data = b"tbt";

        let cipher_text = encrypt_data(data, key.into()).unwrap();
        let decrypted = decrypt_data(&cipher_text, key.into()).unwrap();

        assert_eq!(data[..], decrypted[..]);
    }

    #[test]
    fn note_encryption() {
        use common::note::Note;
        use time::OffsetDateTime;
        use uuid::Uuid;

        let key = b"oeifvncpfiejnvdjpvnwifvj12345678";
        let note = Note {
            id: Uuid::new_v4().into(),
            text: "tbt".into(),
            created_at: OffsetDateTime::now_utc().into(),
            updated_at: OffsetDateTime::now_utc().into(),
            space_id: Uuid::new_v4().into(),
            files: Vec::new(),
        };

        let cipher_text = encrypt(&note, key.into()).unwrap();
        let decrypted_note = decrypt(&cipher_text, key.into()).unwrap();

        assert_eq!(note, decrypted_note);
    }
}
