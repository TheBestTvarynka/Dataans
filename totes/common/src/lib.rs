#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Totes app theme.
///
/// The theme is just a collection of names and values. Every name
/// corresponds to some color name in the app. And the value is an actual color value.
/// For example, _"messages_border"_ is a name, and _"#18191d"_ is a value.
///
/// **Attention**: This structure do not validate anything. It's just a wrapper over
/// the [HashMap]. It is still possible to set invalid color or invalid color name.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Theme(HashMap<String, String>);

impl Theme {
    /// Converts [Theme] to the CSS string.
    pub fn to_css(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("--{}: {};", key, value))
            .collect()
    }
}

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

impl<'text> AsRef<str> for MdText<'text> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Date and time when note was created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreationDate(OffsetDateTime);

impl AsRef<OffsetDateTime> for CreationDate {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

/// Represent one note.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note<'text> {
    /// Note id.
    pub id: Id,
    /// Note data in MD format.
    pub text: MdText<'text>,
    /// Creation date.
    pub created_at: CreationDate,
    // TODO(@TheBestTvarynka): implement attached files, photos, update time etc.
}
