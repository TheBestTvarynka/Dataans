use aes_gcm::aead::Aead;
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sha2::{Digest, Sha256};

pub type Sha256Checksum = [u8; 32];
pub type EncryptionKey = [u8; SERVER_ENCRYPTION_KEY_SIZE];

const NONCE_LENGTH: usize = 12;
const HMAC_SHA256_CHECKSUM_LENGTH: usize = 32;
pub const EMPTY_SHA256_CHECKSUM: &[u8] = &[0; 32];
pub const SERVER_ENCRYPTION_KEY_SIZE: usize = 32;

use crate::{Error, Result};

pub fn sha256(data: &[u8]) -> Sha256Checksum {
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

pub fn encrypt(data: &[u8], key: &EncryptionKey) -> Result<Vec<u8>> {
    // Encryption
    let key = Key::<Aes256Gcm>::from_slice(key);
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
            Error::InvalidKeyLength
        })?;

        mac.update(&nonce);
        mac.update(data);

        mac.finalize().into_bytes()
    };
    result.extend_from_slice(&checksum);

    // result = nonce + cipher_text + checksum
    Ok(result)
}

pub fn decrypt(data: &[u8], key: &EncryptionKey) -> Result<Vec<u8>> {
    // data = nonce + cipher_text + checksum

    if data.len() < NONCE_LENGTH + HMAC_SHA256_CHECKSUM_LENGTH {
        return Err(Error::DecryptionFailed("invalid data length"));
    }

    let (nonce, data) = data.split_at(NONCE_LENGTH);
    let (cipher_text, checksum) = data.split_at(data.len() - HMAC_SHA256_CHECKSUM_LENGTH);

    let nonce = Nonce::from_slice(nonce);

    // Decryption
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let decrypted = cipher.decrypt(nonce, cipher_text)?;

    // Checksum verification
    let expected_checksum = {
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|err| {
            error!(?err, "Failed to initialize HMAC-SHA256");
            Error::InvalidKeyLength
        })?;

        mac.update(nonce);
        mac.update(&decrypted);

        mac.finalize().into_bytes().to_vec()
    };

    if expected_checksum != checksum {
        return Err(Error::DecryptionFailed("message altered"));
    }

    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn encrypt_decrypt() {
        let key = b"oeifvncpfiejnvdjpvnwifvj12345678";
        let data = b"tbt";

        let cipher_text = encrypt(data, key).unwrap();
        let decrypted = decrypt(&cipher_text, key).unwrap();

        assert_eq!(data[..], decrypted[..]);
    }
}
