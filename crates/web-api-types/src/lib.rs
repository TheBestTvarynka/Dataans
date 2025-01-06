mod auth;
mod error;

pub use auth::*;
pub use error::*;
use nutype::nutype;

#[nutype(validate(not_empty), derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom))]
pub struct Username(String);

#[nutype(validate(not_empty), derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom))]
pub struct Password(String);

#[nutype(
    validate(predicate = |token| !token.is_empty()),
    derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom),
)]
pub struct InvitationToken(Vec<u8>);
