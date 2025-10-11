//! Crypto-related primitives for the app.
//!
//! This module contains all needed code for the data hashing, encryption,
//! decryption, HMAC, etc.
//!
//! # Encryption
//!
//! The user's data is encrypted before sending to the sync server. The encryption algorithm is [AES GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode).
//! AES GCM needs a nonce. The nonce is randomly generated and prepended to the resulting vector.
//! Additionally, the HMAC-SHA256 is computed over the unencrypted data and appended to the resulting
//! byte vector.
//!
//! # Encryption key derivation
//!
//! To derive the encryption key, the app needs the user's password and the special passphrase (salt).
//! The passphrase (salt) is randomly generated during the first sign in. The user must use the same passphrase
//! on all subsequential sign ins. This salt is stored in the `profile.json` file in the user's folder.
//!
//! The user is responsible for storing their password. The app never stores the user's password anywhere.
//!
//! The key derivation algorithm is [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2). Iteration count is hardcoded
//! and is equal to 1_200_000.
//!
//! The password and the salt can be very long, so they are hashed using the SHA256 before passing into PBKDF2.
//! ```
//! let key = pbkdf2(sha256(password), sha256(salt), 1_200_000);
//! ```

use aes_gcm::aead::Aead;
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, KeySizeUser, Nonce};
use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::pbkdf2_hmac;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::digest::typenum::Unsigned;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// AES-GCM 96-bit (12-byte) nonce.
const NONCE_LENGTH: usize = <Aes256Gcm as AeadCore>::NonceSize::USIZE;
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

    #[error("OS error: {0}")]
    Os(String),

    #[error("argon2 hash error: {0}")]
    Argon2Hash(#[from] argon2::password_hash::Error),
}

type CryptoResult<T> = Result<T, CryptoError>;

/// Encrypts the data using the provided encryption key.
pub fn encrypt_data(data: &[u8], key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    // Encryption
    let nonce = Aes256Gcm::generate_nonce().map_err(|err| CryptoError::Os(err.to_string()))?;
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

/// Decrypts the data using the provided encryption key.
pub fn decrypt_data(data: &[u8], key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    // data = nonce + cipher_text + checksum

    if data.len() < NONCE_LENGTH + HMAC_SHA256_CHECKSUM_LENGTH {
        return Err(CryptoError::DecryptionFailed("invalid data length"));
    }

    let (nonce, data) = data.split_at(NONCE_LENGTH);
    let (cipher_text, checksum) = data.split_at(data.len() - HMAC_SHA256_CHECKSUM_LENGTH);

    let nonce = Nonce::try_from(nonce).expect("nonce length is always correct");

    // Decryption
    let cipher = Aes256Gcm::new(key);

    let decrypted = cipher.decrypt(&nonce, cipher_text)?;

    // Checksum verification
    let expected_checksum = {
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|err| {
            error!(?err, "Failed to initialize HMAC-SHA256");
            CryptoError::InvalidKeyLength
        })?;

        mac.update(&nonce);
        mac.update(&decrypted);

        mac.finalize().into_bytes().to_vec()
    };

    if expected_checksum != checksum {
        return Err(CryptoError::DecryptionFailed("message altered"));
    }

    Ok(decrypted)
}

/// The same as [encrypt_data], but accepts the serializable object instead of byte slice.
///
/// This is a helper function, so the user does not have to serialize the object manually every time.
pub fn encrypt<T: Serialize>(data: &T, key: &EncryptionKey) -> CryptoResult<Vec<u8>> {
    let data = serde_json::to_vec(data)?;

    encrypt_data(&data, key)
}

/// The same as [decrypt_data], but returns the deserialized object instead of byte vector.
///
/// This is a helper function, so the user does not have to deserialize the object manually every time.
pub fn decrypt<T: DeserializeOwned>(data: &[u8], key: &EncryptionKey) -> CryptoResult<T> {
    let data = decrypt_data(data, key)?;

    Ok(serde_json::from_slice(&data)?)
}

/// Derives the encryption key for encrypting the user's data.
pub fn derive_encryption_key(password: &[u8], salt: &[u8]) -> CryptoResult<EncryptionKey> {
    let password = Sha256::digest(password).to_vec();
    let salt = Sha256::digest(salt).to_vec();

    let mut key = [0; <Aes256Gcm as KeySizeUser>::KeySize::USIZE];
    pbkdf2_hmac::<Sha256>(&password, &salt, 1_200_000, &mut key);

    Ok(key.into())
}

/// Computes argon2 hash of the encryption key.
pub fn hash_encryption_key(key: &EncryptionKey) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(key.as_ref(), &salt)?.to_string();

    Ok(hash)
}

/// Verifies that the provided encryption key matches the hash.
pub fn verify_encryption_key_hash(key: &EncryptionKey, hash: &str) -> Result<(), CryptoError> {
    let parsed_hash = PasswordHash::new(hash)?;

    Argon2::default().verify_password(key.as_ref(), &parsed_hash)?;

    Ok(())
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
