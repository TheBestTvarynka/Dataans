mod auth;
mod data;

pub use auth::*;
pub use data::*;
use rocket::get;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use web_api_types::{UserId, AUTH_COOKIE_NAME, AUTH_HEADER_NAME};

use crate::{Error, WebServerState};

#[get("/")]
pub fn health() -> &'static str {
    "ok"
}

#[derive(Debug)]
pub struct UserContext {
    #[allow(dead_code)]
    pub user_id: UserId,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserContext {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = if let Some(token) = req.cookies().get(AUTH_COOKIE_NAME) {
            token.value()
        } else {
            match req
                .headers()
                .get_one(AUTH_HEADER_NAME)
                .ok_or_else(|| Error::Session("missing token"))
            {
                Ok(token) => token,
                Err(err) => return Outcome::Error((Status::Unauthorized, err)),
            }
        };

        let state = match req
            .rocket()
            .state::<WebServerState>()
            .ok_or_else(|| Error::Internal("missing Rocket state"))
        {
            Ok(state) => state,
            Err(err) => return Outcome::Error((Status::InternalServerError, err)),
        };
        let user_id = match state.auth_service.verify_session(token).await {
            Ok(user_id) => user_id,
            Err(err) => return Outcome::Error((Status::Unauthorized, err)),
        };

        Outcome::Success(UserContext { user_id })
    }
}
