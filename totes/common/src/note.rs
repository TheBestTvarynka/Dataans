use std::borrow::Cow;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::space::Id as SpaceId;
use crate::CreationDate;

/// Represent a note ID.
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

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// Represent a note text.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct MdText<'text>(Cow<'text, str>);

impl From<String> for MdText<'static> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'text> From<&'text str> for MdText<'text> {
    fn from(value: &'text str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'text> AsRef<str> for MdText<'text> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represent one note.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Note<'text> {
    /// Note id.
    pub id: Id,
    /// Note data in MD format.
    pub text: MdText<'text>,
    /// Creation date.
    pub created_at: CreationDate,
    /// Space ID this note belongs.
    pub space_id: SpaceId,
    // TODO(@TheBestTvarynka): implement attached files, photos, update time etc.
}

/// Represent note to update.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UpdateNote<'text> {
    /// Note id.
    pub id: Id,
    /// Updated note text.
    pub text: MdText<'text>,
}
