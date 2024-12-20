use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::dataans::db::DbError;

#[derive(Debug, Error, Serialize)]
pub enum DataansError {
    #[serde(serialize_with = "serialize_db_error")]
    #[error("DbError: {0:?}")]
    DbError(#[from] DbError),
}

pub fn serialize_db_error<S>(db_error: &DbError, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{:?}", db_error))
}
