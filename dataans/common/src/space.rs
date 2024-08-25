use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::CreationDate;

/// Represent a space ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
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

impl From<Name<'_>> for String {
    fn from(value: Name<'_>) -> Self {
        match value.0 {
            Cow::Borrowed(s) => s.to_owned(),
            Cow::Owned(s) => s,
        }
    }
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
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

/// Represents space avatar file name.
///
/// Example: `461d7188-062a-4514-bece-3577624d0ee8.png`.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Avatar<'avatar>(Cow<'avatar, str>);

impl From<String> for Avatar<'static> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Display for Avatar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl<'name> AsRef<str> for Avatar<'name> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represents a space.
///
/// Space - a collection of notes.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Space<'name, 'avatar> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Creation date.
    pub created_at: CreationDate,
    /// Avatar image name.
    pub avatar: Avatar<'avatar>,
}

/// Owned version of [Space].
pub type OwnedSpace = Space<'static, 'static>;

/// Data that the app need to update the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSpace<'name> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Space avatar.
    pub avatar: Avatar<'static>,
}

/// Data that the app need to delete the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteSpace {
    /// Space ID.
    pub id: Id,
}
