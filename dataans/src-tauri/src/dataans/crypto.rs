use aes_gcm::aead::Aead;
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use thiserror::Error;

const NONCE_LENGTH: usize = 12;
const HMAC_SHA256_CHECKSUM_LENGTH: usize = 32;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid encryption key or IV length")]
    InvalidKeyLength,

    #[error("encryption error: {0}")]
    Encryption(#[from] aes_gcm::Error),

    #[error("failed to decrypt the data: {0}")]
    DecryptionFailed(&'static str),
}

type CryptoResult<T> = Result<T, CryptoError>;

pub fn encrypt_data(data: &[u8], key: &Key<Aes256Gcm>) -> CryptoResult<Vec<u8>> {
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


pub fn decrypt_data(data: &[u8], key: &Key<Aes256Gcm>) -> CryptoResult<Vec<u8>> {
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
}