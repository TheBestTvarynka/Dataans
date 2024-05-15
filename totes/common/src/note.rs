use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::CreationDate;

/// Represent a note ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub struct Id(u32);

impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Represent a note text.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MdText<'text>(Cow<'text, str>);

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note<'text> {
    /// Note id.
    pub id: Id,
    /// Note data in MD format.
    pub text: MdText<'text>,
    /// Creation date.
    pub created_at: CreationDate,
    // TODO(@TheBestTvarynka): implement attached files, photos, update time etc.
}
