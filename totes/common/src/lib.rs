#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Contains all note-related structures.
pub mod note;
/// Contains all space-related structures.
pub mod space;

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

/// Date and time when note was created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreationDate(OffsetDateTime);

impl From<OffsetDateTime> for CreationDate {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl AsRef<OffsetDateTime> for CreationDate {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}
