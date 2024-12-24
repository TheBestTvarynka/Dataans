use std::borrow::Cow;
use std::fmt::Display;
use std::path::PathBuf;

use bson::Bson;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::space::{Id as SpaceId, Space};
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

impl Display for MdText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

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

impl<'text> From<MdText<'text>> for String {
    fn from(value: MdText<'text>) -> String {
        match value.0 {
            Cow::Borrowed(s) => s.to_owned(),
            Cow::Owned(s) => s,
        }
    }
}

impl AsRef<str> for MdText<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represents an uploaded file.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct File {
    /// The unique file id.
    pub id: Uuid,
    /// The original file name.
    pub name: String,
    /// Full path to the file in the local file system.
    pub path: PathBuf,
}

impl From<File> for Bson {
    fn from(file: File) -> Bson {
        bson::to_bson(&file).expect("should not fail")
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
    /// Attached files.
    pub files: Vec<File>,
}

/// Owned version of [Note].
pub type OwnedNote = Note<'static>;

/// Represents draft note.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct DraftNote<'text> {
    /// Note data in MD format.
    pub text: MdText<'text>,
    /// Attached files.
    pub files: Vec<File>,
}

/// Represent one note.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct NoteFull<'text, 'space_name, 'space_avatar> {
    /// Note id.
    pub id: Id,
    /// Note data in MD format.
    pub text: MdText<'text>,
    /// Creation date.
    pub created_at: CreationDate,
    /// Space ID this note belongs.
    pub space: Space<'space_name, 'space_avatar>,
    /// Attached files.
    pub files: Vec<File>,
}

/// Owned version of the [NoteFull] type.
pub type NoteFullOwned = NoteFull<'static, 'static, 'static>;

/// Represent note to update.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UpdateNote<'text> {
    /// Note id.
    pub id: Id,
    /// Updated note text.
    pub text: MdText<'text>,
    /// Attached files.
    pub files: Vec<File>,
}
