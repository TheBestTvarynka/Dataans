use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::note::File;
use crate::{CreationDate, UpdateDate};

/// Represent a space ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Id(Uuid);

impl Id {
    /// Returns the inner ID.
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        value.0
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

impl AsRef<str> for Name<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represents space avatar file name.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Avatar<'avatar> {
    id: Uuid,
    path: Cow<'avatar, str>,
}

impl<'avatar> Avatar<'avatar> {
    /// Creates a new [Avatar] based on `id` and `path`.
    pub fn new(id: Uuid, path: impl Into<Cow<'avatar, str>>) -> Self {
        Self { id, path: path.into() }
    }

    /// Returns avatar [Uuid].
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns path to the avatar file.
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}

impl From<File> for Avatar<'_> {
    fn from(file: File) -> Self {
        let File { id, name: _, path } = file;
        Self {
            id,
            path: path.to_str().expect("UTF8-path").to_owned().into(),
        }
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
    /// Update date.
    pub updated_at: UpdateDate,
    /// Avatar image name.
    pub avatar: Avatar<'avatar>,
}

/// Owned version of [Space].
pub type OwnedSpace = Space<'static, 'static>;

/// Data that the app need to create the space.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CreateSpace<'name, 'avatar> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Avatar image name.
    pub avatar: Avatar<'avatar>,
}

/// Owned version of [CreateSpace];
pub type CreateSpaceOwned = CreateSpace<'static, 'static>;

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
