use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::CreationDate;

/// Represent a space ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Id(Uuid);

impl Id {
    /// Returns the inner ID.
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

/// Represents a space name.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Name<'name>(Cow<'name, str>);

impl From<String> for Name<'static> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'name> From<&'name str> for Name<'name> {
    fn from(value: &'name str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'name> AsRef<str> for Name<'name> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represents a space.
///
/// Space - a collection of notes.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Space<'name> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Creation date.
    pub created_at: CreationDate,
    // TODO(@TheBestTvarynka): implement space avatar image.
}

/// Data that the app need to update the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSpace<'name> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
}

/// Data that the app need to delete the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteSpace {
    /// Space ID.
    pub id: Id,
}
