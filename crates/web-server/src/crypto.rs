use aes_gcm::aead::Aead;
use aes_gcm::{AeadCore, Aes256Gcm, Key};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sha2::{Digest, Sha256};

pub type SHA256Checksum = [u8; 32];
pub const EMPTY_SHA256_CHECKSUM: &[u8] = &[0; 32];

use crate::{Error, Result};

pub fn sha256(data: &[u8]) -> SHA256Checksum {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize().into()
}

pub fn hash_password(password: &[u8]) -> Result<Vec<u8>> {
    let password = sha256(password);
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    Ok(argon2.hash_password(&password, &salt)?.to_string().into_bytes())
}

pub fn verify_password(password: &[u8], hash: &[u8]) -> Result<()> {
    let password = sha256(password);
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(std::str::from_utf8(hash).map_err(|err| {
        error!(?err, "Failed to parse hash");
        Error::PasswordHashParsingError
    })?)?;

    Ok(argon2.verify_password(&password, &parsed_hash)?)
}

pub fn hmac(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};

    let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|err| {
        error!(?err, "Failed to initialize HMAC-SHA256");
        Error::InvalidKeyLength
    })?;

    mac.update(data);

    Ok(mac.finalize().into_bytes().to_vec())
}

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    use aes_gcm::KeyInit;

    // Encryption
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let cipher_text = cipher.encrypt(&nonce, data)?;

    let mut result = nonce.as_slice().to_vec();
    result.extend_from_slice(&cipher_text);

    // Checksum
    let checksum = hmac(&result, key)?;
    result.extend_from_slice(&checksum);

    // result = nonce + cipher_text + checksum
    Ok(result)
}
