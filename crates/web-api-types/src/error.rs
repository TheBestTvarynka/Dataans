use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(rocket::Responder))]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    DbError(String),

    #[cfg_attr(feature = "server", response(status = 404, content_type = "json"))]
    NotFound(String),

    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    PasswordHashingError(String),

    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    UnableToVerifyCredentials(String),

    #[cfg_attr(feature = "server", response(status = 401, content_type = "json"))]
    InvalidCredentials(String),

    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    Internal(String),

    #[cfg_attr(feature = "server", response(status = 403, content_type = "json"))]
    AccessDenied(String),

    #[cfg_attr(feature = "server", response(status = 400, content_type = "json"))]
    InvalidData(String),

    #[cfg_attr(feature = "server", response(status = 401, content_type = "json"))]
    Unauthorized(String),
}

pub type Result<T> = std::result::Result<T, Error>;
