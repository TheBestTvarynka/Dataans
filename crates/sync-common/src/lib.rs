use sha2::digest::generic_array::GenericArray;
use sha2::Digest;
pub use sha2::Sha256;
use time::OffsetDateTime;
use uuid::Uuid;

pub trait Hasher: Digest {
    fn hash_str(&mut self, data: impl AsRef<str>) {
        self.update(data.as_ref().as_bytes());
    }
}

impl Hasher for Sha256 {}

pub trait Hash {
    fn hash<H: Hasher>(&self, hasher: &mut H);

    fn digest<H: Hasher>(&self) -> GenericArray<u8, H::OutputSize> {
        let mut hasher = H::new();

        self.hash(&mut hasher);

        hasher.finalize()
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
