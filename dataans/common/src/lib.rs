#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Contains general error and result types for Tauri commands.
pub mod error;
/// Events names and types.
pub mod event;
/// Contains schema definitions for data export.
pub mod export;
/// All possible frontend keybindings definitions.
pub mod key_bindings;
/// Contains all note-related structures.
pub mod note;
/// User's profile.
pub mod profile;
/// Contains all space-related structures.
pub mod space;

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use derive_more::{AsRef, From, Into};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::{Uuid, uuid};
pub use web_api_types as common_api_types;

use crate::export::SchemaVersion;

/// Name of the custom tauri plugin.
pub const APP_PLUGIN_NAME: &str = "dataans";

/// Default space avatar file id.
///
/// It's just a random UUID. Nothing special.
pub const DEFAULT_SPACE_AVATAR_ID: Uuid = uuid!("54d49bda-644e-44a9-a1ad-4a8fa5f368a5");
/// Default space avatar file path.
pub const DEFAULT_SPACE_AVATAR_PATH: &str = "/public/default_space_avatar.png";

/// Dataans app theme.
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
        self.0.iter().fold(String::new(), |mut css, (key, value)| {
            use std::fmt::Write;

            let _ = write!(css, "--{key}: {value};");
            css
        })
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

/// App configuration.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct App {
    /// App toggle: show/hide app.
    #[serde(default = "app_toggle")]
    pub app_toggle: String,
    /// Always on top.
    #[serde(default = "always_on_top")]
    pub always_on_top: bool,
    /// Hide app window decorations.
    #[serde(default = "hide_window_decorations")]
    pub hide_window_decorations: bool,
    /// Hide app icon on taskbar.
    #[serde(default = "hide_taskbar_icon")]
    pub hide_taskbar_icon: bool,
    /// Base path for the all app data: config file, user files, DB, etc.
    #[serde(default)]
    pub base_path: String,
}

fn hide_taskbar_icon() -> bool {
    false
}

fn hide_window_decorations() -> bool {
    false
}

fn always_on_top() -> bool {
    false
}

fn app_toggle() -> String {
    "F2".into()
}

/// Represents app configuration.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Defined key bindings.
    pub key_bindings: key_bindings::KeyBindings,
    /// Appearance configuration options.
    pub appearance: Appearance,
    /// App configuration.
    pub app: App,
}

/// Date and time when the item was created.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, From, Into, AsRef, PartialOrd, Ord)]
pub struct CreationDate(OffsetDateTime);

/// Date and time when the item was updated.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, From, Into, AsRef, PartialOrd, Ord)]
pub struct UpdateDate(OffsetDateTime);

/// Option that describes how to export notes.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum NotesExportOption {
    /// All exported data will be in one `.md` file.
    #[default]
    OneFile,
    /// For each space a separate file will be created.
    FilePerSpace,
    /// For each note a separate file will be created. All these files will be grouped by folders which represent spaces.
    FilePerNote,
}

impl NotesExportOption {
    /// Returns a slice that contains all [NotesExportOption] variants.
    pub fn variants() -> &'static [NotesExportOption] {
        &[
            NotesExportOption::OneFile,
            NotesExportOption::FilePerSpace,
            NotesExportOption::FilePerNote,
        ]
    }

    /// Returns [NotesExportOption] variant name.
    pub fn variant_name(&self) -> &'static str {
        match self {
            NotesExportOption::OneFile => "OneFile",
            NotesExportOption::FilePerSpace => "FilePerSpace",
            NotesExportOption::FilePerNote => "FilePerNote",
        }
    }

    /// Returns pretty name of [NotesExportOption].
    pub fn pretty(&self) -> &'static str {
        match self {
            NotesExportOption::OneFile => "One file",
            NotesExportOption::FilePerSpace => "File per space",
            NotesExportOption::FilePerNote => "File per note",
        }
    }

    /// Creates [NotesExportOption] from the `str`.
    ///
    /// Panic: on invalid value.
    pub fn _from_str(value: &str) -> Self {
        match value {
            "OneFile" => NotesExportOption::OneFile,
            "FilePerSpace" => NotesExportOption::FilePerSpace,
            "FilePerNote" => NotesExportOption::FilePerNote,
            _ => panic!("Invalid NotesExportOption value: {value}"),
        }
    }
}

impl fmt::Display for NotesExportOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.variant_name())
    }
}

/// Configuration for app data export.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum DataExportConfig {
    /// Markdown format with its options.
    Md(NotesExportOption),
    /// Json export format with its options.
    Json(SchemaVersion),
}

impl DataExportConfig {
    /// Returns all possible [DataExportConfig] variants initialized with default values.
    pub fn variants() -> &'static [DataExportConfig] {
        &[
            DataExportConfig::Md(NotesExportOption::OneFile),
            DataExportConfig::Json(SchemaVersion::V1),
        ]
    }

    /// Creates [NotesExportOption] from the `str` variant name.
    ///
    /// Panic: on invalid value.
    pub fn _from_str(value: &str) -> Self {
        match value {
            "Md" => DataExportConfig::Md(Default::default()),
            "Json" => DataExportConfig::Json(Default::default()),
            _ => panic!("Invalid DataExportConfig variant name: {value}"),
        }
    }

    /// Returns name of the [DataExportConfig] variant.
    pub fn variant_name(&self) -> &'static str {
        match self {
            DataExportConfig::Md(_) => "Md",
            DataExportConfig::Json(_) => "Json",
        }
    }
}

impl Default for DataExportConfig {
    fn default() -> Self {
        Self::Json(Default::default())
    }
}
