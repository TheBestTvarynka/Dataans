use core::fmt;
use std::iter::Peekable;

use serde::{Deserialize, Serialize};

/// Represents all defined keybindings.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct KeyBindings {
    /// Toggle spaces bar.
    #[serde(default = "toggle_spaces_bar")]
    pub toggle_spaces_bar: KeyBinding,
    /// Create space.
    #[serde(default = "create_space")]
    pub create_space: KeyBinding,
    /// Edit current space.
    #[serde(default = "edit_current_space")]
    pub edit_current_space: KeyBinding,
    /// Delete current space.
    #[serde(default = "delete_current_space")]
    pub delete_current_space: KeyBinding,
    /// Select previous space.
    #[serde(default = "select_next_list_item")]
    pub select_next_list_item: KeyBinding,
    /// Select next space.
    #[serde(default = "select_prev_list_item")]
    pub select_prev_list_item: KeyBinding,
    /// Find note.
    #[serde(default = "find_note")]
    pub find_note: KeyBinding,
    /// Find note in the selected space.
    #[serde(default = "find_note_in_selected_space")]
    pub find_note_in_selected_space: KeyBinding,
    /// Regenerate space avatar image.
    #[serde(default = "regenerate_space_avatar")]
    pub regenerate_space_avatar: KeyBinding,
}

fn regenerate_space_avatar() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "R".into(),
    }
}

fn find_note_in_selected_space() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "M".into(),
    }
}

fn find_note() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "F".into(),
    }
}

fn select_next_list_item() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            alt: true,
            ..Default::default()
        },
        key: "2".into(),
    }
}

fn select_prev_list_item() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            alt: true,
            ..Default::default()
        },
        key: "1".into(),
    }
}

fn toggle_spaces_bar() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "S".into(),
    }
}

fn create_space() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "N".into(),
    }
}

fn edit_current_space() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "E".into(),
    }
}

fn delete_current_space() -> KeyBinding {
    KeyBinding {
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
        key: "Del".into(),
    }
}

/// Possible keybinding modifiers.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KeyModifiers {
    /// True if the Control key is pressed.
    pub ctrl: bool,
    /// True if the Shift key is pressed.
    pub shift: bool,
    /// True if the Alt key is pressed.
    pub alt: bool,
    /// True if the Meta key is pressed.
    pub meta: bool,
}

impl KeyModifiers {
    /// Returns true if no modifiers.
    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.shift && !self.alt && !self.meta
    }

    /// Returns a new instance of [KeyModifiers] from a peekable iterator.
    pub fn from_peekable<'a>(parts: &mut Peekable<impl Iterator<Item = &'a str>>) -> Self {
        let mut ctrl = false;
        let mut shift = false;
        let mut alt = false;
        let mut meta = false;

        while let Some(modifier) = parts.peek() {
            match *modifier {
                "Ctrl" => ctrl = true,
                "Shift" => shift = true,
                "Alt" => alt = true,
                "Meta" => meta = true,
                _ => break,
            }

            parts.next();
        }

        Self { ctrl, shift, alt, meta }
    }
}

impl fmt::Display for KeyModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_first_modifier = true;

        if self.ctrl {
            write!(f, "Ctrl")?;
            is_first_modifier = false;
        }

        if self.shift {
            if is_first_modifier {
                write!(f, "Shift")?;
                is_first_modifier = false;
            } else {
                write!(f, "+Shift")?;
            }
        }

        if self.alt {
            if is_first_modifier {
                write!(f, "Alt")?;
                is_first_modifier = false;
            } else {
                write!(f, "+Alt")?;
            }
        }

        if self.meta {
            if is_first_modifier {
                write!(f, "Meta")?;
            } else {
                write!(f, "+Meta")?;
            }
        }

        Ok(())
    }
}

/// Represents a single key binding.
///
/// The key binding representation: modifiers + key.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KeyBinding {
    /// Key binding modifiers.
    pub modifiers: KeyModifiers,
    /// Key binding key.
    pub key: String,
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.modifiers.fmt(f)?;

        if !self.modifiers.is_empty() {
            write!(f, "+")?;
        }

        self.key.fmt(f)
    }
}

impl<'de> Deserialize<'de> for KeyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_key_binding = String::deserialize(deserializer)?;

        let mut parts = raw_key_binding.split('+').peekable();

        let modifiers = KeyModifiers::from_peekable(&mut parts);
        let key = parts
            .next()
            .ok_or_else(|| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&raw_key_binding),
                    &"invalid key binding: missing key part",
                )
            })?
            .to_owned();

        if parts.next().is_some() {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&raw_key_binding),
                &"invalid key binding: too many keys provided",
            ));
        }

        Ok(KeyBinding { modifiers, key })
    }
}

impl Serialize for KeyBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.modifiers.to_string();

        if !self.modifiers.is_empty() {
            s.push('+');
        }
        s.push_str(&self.key);

        serializer.serialize_str(&s)
    }
}
