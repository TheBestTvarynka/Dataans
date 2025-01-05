use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use sha2::{Digest, Sha256};

use crate::Result;

pub fn sha256(data: &[u8]) -> [u8; 32] {
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
