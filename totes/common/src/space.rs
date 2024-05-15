use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::CreationDate;

/// Represent a space ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub struct Id(u32);

impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Represents a space name.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Name<'name>(Cow<'name, str>);

impl<'name> From<&'name str> for Name<'name> {
    fn from(value: &'name str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

/// Represents a space.
///
/// Space - a collection of notes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Space<'name> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Creation date.
    pub created_at: CreationDate,
    // TODO(@TheBestTvarynka): implement space avatar image.
}
