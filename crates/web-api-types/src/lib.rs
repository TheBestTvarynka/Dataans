mod data;
mod error;

pub use data::*;
use derive_more::{AsRef, From, Into};
pub use error::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct OperationId(uuid::Uuid);

#[cfg(feature = "server")]
mod impl_from_param {
    use rocket::request::FromParam;
    use uuid::Uuid;

    macro_rules! impl_from_param {
        (id: $id_type:ty) => {
            impl<'a> FromParam<'a> for $id_type {
                type Error = <Uuid as FromParam<'a>>::Error;

                fn from_param(param: &str) -> Result<Self, Self::Error> {
                    Uuid::from_param(param).map(From::from)
                }
            }
        };
    }

    impl_from_param!(id: crate::OperationId);
}

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct OperationChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct OperationData(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct CreationDate(#[serde(with = "rfc3339")] OffsetDateTime);
