//! Hashing primitives.
//!
//! Data objects like notes and spaces need to be hashed during the sync process.
//! Object hash calculation can be easily customized in the [Hash] trait implementation.
//! For example, some fields are skipped and do not count in hash calculation.

use sha2::digest::generic_array::GenericArray;
use sha2::Digest;
pub use sha2::Sha256;
use time::OffsetDateTime;
use uuid::Uuid;

/// Custom [Digest] trait extension.
pub trait Hasher: Digest {
    fn hash_str(&mut self, data: impl AsRef<str>) {
        self.update(data.as_ref().as_bytes());
    }
}

impl Hasher for Sha256 {}

/// Hashable type.
///
/// Any type that implement the [Hash] trait can be hashed.
pub trait Hash {
    /// Hashed the `self`.
    fn hash<H: Hasher>(&self, hasher: &mut H);

    /// Returns the calculated hash over the `self`.
    fn digest<H: Hasher>(&self) -> GenericArray<u8, H::OutputSize> {
        let mut hasher = H::new();

        self.hash(&mut hasher);

        hasher.finalize()
    }
}

impl Hash for bool {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.update([u8::from(*self)]);
    }
}

impl Hash for &str {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.hash_str(self);
    }
}

impl Hash for String {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.hash_str(self);
    }
}

impl Hash for Uuid {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.update(self.as_bytes());
    }
}

impl Hash for OffsetDateTime {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.update(self.unix_timestamp_nanos().to_be_bytes());
    }
}

impl<T: Hash> Hash for &[T] {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for hashable in self.iter() {
            hashable.hash(hasher);
        }
    }
}
