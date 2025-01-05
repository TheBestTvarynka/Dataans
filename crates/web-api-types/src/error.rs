use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(rocket::Responder))]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    DbError(String),

    #[cfg_attr(feature = "server", response(status = 404, content_type = "json"))]
    InvitationTokenNotFound(Vec<u8>),

    #[cfg_attr(feature = "server", response(status = 500, content_type = "json"))]
    PasswordHashingError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
