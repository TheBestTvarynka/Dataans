mod auth;
mod data;
mod error;
mod sync;

pub use auth::*;
pub use data::*;
use derive_more::{AsRef, From, Into};
pub use error::*;
use nutype::nutype;
use serde::{Deserialize, Serialize};
pub use sync::*;

#[nutype(
    validate(not_empty),
    derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom, Clone)
)]
pub struct Username(String);

#[nutype(
    validate(not_empty),
    derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom, Clone)
)]
pub struct Password(String);

#[nutype(
    validate(predicate = |token| !token.is_empty()),
    derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom),
)]
pub struct InvitationToken(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct UserId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct BlockId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct NoteId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct SpaceId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq, Hash)]
pub struct SessionId(uuid::Uuid);

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

    impl_from_param!(id: crate::SpaceId);
    impl_from_param!(id: crate::NoteId);
    impl_from_param!(id: crate::UserId);
    impl_from_param!(id: crate::SessionId);
}

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct NoteChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct NoteData(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct BlockChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct SpaceChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct FileChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct SpaceData(Vec<u8>);
