mod auth;
mod error;
mod sync;

pub use auth::*;
use derive_more::{AsRef, From};
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

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct UserId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct BlockId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct NoteId(uuid::Uuid);

#[derive(Debug, Serialize, Deserialize, AsRef, From)]
pub struct SpaceId(uuid::Uuid);
