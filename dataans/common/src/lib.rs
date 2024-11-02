#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Contains all note-related structures.
pub mod note;
/// Contains all space-related structures.
pub mod space;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Name of the custom tauri plugin.
pub const APP_PLUGIN_NAME: &str = "dataans";

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

            let _ = write!(css, "--{}: {};", key, value);
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

/// Represents all defined keybindings.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct KeyBindings {
    /// Toggle spaces bar.
    #[serde(default = "toggle_spaces_bar")]
    pub toggle_spaces_bar: String,
    /// Create space.
    #[serde(default = "create_space")]
    pub create_space: String,
    /// Edit current space.
    #[serde(default = "edit_current_space")]
    pub edit_current_space: String,
    /// Delete current space.
    #[serde(default = "delete_current_space")]
    pub delete_current_space: String,
    /// Select previous space.
    #[serde(default = "select_next_list_item")]
    pub select_next_list_item: String,
    /// Select next space.
    #[serde(default = "select_prev_list_item")]
    pub select_prev_list_item: String,
    /// Find note.
    #[serde(default = "find_note")]
    pub find_note: String,
    /// Find note in the selected space.
    #[serde(default = "find_note_in_selected_space")]
    pub find_note_in_selected_space: String,
}

fn find_note_in_selected_space() -> String {
    "ControlLeft+keyM".into()
}

fn find_note() -> String {
    "ControlLeft+keyF".into()
}

fn select_next_list_item() -> String {
    "AltLeft+Digit2".into()
}

fn select_prev_list_item() -> String {
    "AltLeft+Digit1".into()
}

fn toggle_spaces_bar() -> String {
    "ControlLeft+keyS".into()
}

fn create_space() -> String {
    "ControlLeft+keyN".into()
}

fn edit_current_space() -> String {
    "ControlLeft+keyE".into()
}

fn delete_current_space() -> String {
    "ControlLeft+keyE".into()
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
    pub key_bindings: KeyBindings,
    /// Appearance configuration options.
    pub appearance: Appearance,
    /// App configuration.
    pub app: App,
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
