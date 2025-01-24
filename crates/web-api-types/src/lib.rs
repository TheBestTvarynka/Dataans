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

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq)]
pub struct UserId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq)]
pub struct BlockId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq)]
pub struct NoteId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Copy, Clone, Into, PartialEq, Eq)]
pub struct SpaceId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct NoteChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct NoteData(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct BlockChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct SpaceChecksumValue(Vec<u8>);

#[derive(Debug, Serialize, Deserialize, AsRef, From, Into)]
pub struct SpaceData(Vec<u8>);
