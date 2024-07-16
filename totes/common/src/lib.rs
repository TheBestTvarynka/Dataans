#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Contains all note-related structures.
pub mod note;
/// Contains all space-related structures.
pub mod space;

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Name of the custom tauri plugin.
pub const TOTES_PLUGIN_NAME: &str = "totes";

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

/// Represents all possible appearance configuration options.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Appearance {
    /// Path to the theme file.
    ///
    /// Example: `theme_dark.toml`, `my_custom/dark.toml`.
    #[serde(default = "theme")]
    pub theme: PathBuf,
}

fn theme() -> PathBuf {
    PathBuf::from("theme_dark.toml")
}

/// Represents all defined keybindings.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct KeyBindings {
    /// Toggle spaces bar.
    #[serde(default = "toggle_spaces_bar")]
    pub toggle_spaces_bar: String,
}

fn toggle_spaces_bar() -> String {
    "ControlLeft+keyS".into()
}

/// Represents app configuration.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Defined key bindings.
    pub key_bindings: KeyBindings,
    /// Appearance configuration options.
    pub appearance: Appearance,
}

/// Date and time when note was created.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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
