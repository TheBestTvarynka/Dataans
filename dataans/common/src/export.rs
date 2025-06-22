use std::fmt;

use serde::{Deserialize, Serialize};

use crate::note::OwnedNote;
use crate::space::OwnedSpace;

/// Schema version.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum SchemaVersion {
    /// V1.
    #[default]
    V1,
}

impl SchemaVersion {
    /// Returns slice that contains all possible schema versions.
    pub fn variants() -> &'static [SchemaVersion] {
        &[SchemaVersion::V1]
    }

    /// Returns [SchemaVersion] variant name.
    pub fn variant_name(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "V1",
        }
    }

    /// Creates [SchemaVersion] from the `str`.
    ///
    /// Panic: on invalid value.
    pub fn _from_str(value: &str) -> Self {
        match value {
            "V1" => SchemaVersion::V1,
            _ => panic!("Invalid export schema version: {value}"),
        }
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.variant_name())
    }
}

/// Json data export schema and the data itself.
#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum Schema {
    /// V1.
    V1(SchemaV1),
}

/// App data. V1.
#[derive(Serialize, Deserialize)]
pub struct SchemaV1 {
    /// App data.
    pub data: Vec<Space>,
}

/// Space data.
#[derive(Serialize, Deserialize)]
pub struct Space {
    /// Space info.
    pub space: OwnedSpace,
    /// Spaces notes.
    pub notes: Vec<OwnedNote>,
}
