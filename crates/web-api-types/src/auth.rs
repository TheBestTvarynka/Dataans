use derive_more::{AsRef, Deref, From, Into};
use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::OffsetDateTime;

use super::*;

pub const AUTH_COOKIE_NAME: &str = "dataans-auth";
pub const AUTH_HEADER_NAME: &str = "Authorization";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpRequest {
    pub invitation_token: InvitationToken,
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, Serialize, Deserialize, AsRef, Deref, From, Into, Clone)]
pub struct AuthToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInResponse {
    pub user_id: UserId,
    pub token: AuthToken,
    #[serde(with = "rfc3339")]
    pub expiration_date: OffsetDateTime,
}

#[cfg(feature = "server")]
mod impl_responder {
    use rocket::http::{ContentType, Cookie, Status};
    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};
    use rocket::serde::json::to_string;

    use super::SignInResponse;

    impl<'r> Responder<'r, 'static> for SignInResponse {
        fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
            let SignInResponse {
                user_id: _,
                token,
                expiration_date: _,
            } = &self;

            let body = to_string(&self).map_err(|_| Status::InternalServerError)?.into_bytes();

            Response::build_from(body.respond_to(req)?)
                .status(Status::Ok)
                .header(ContentType::JSON)
                .header(
                    Cookie::build((crate::AUTH_COOKIE_NAME, token.as_str()))
                        .domain(env!("DATAANS_SERVER_DOMAIN"))
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .build(),
                )
                .ok()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub expiration_date: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let data = SignUpRequest {
            invitation_token: InvitationToken::try_from(vec![1, 2, 3, 4]).unwrap(),
            username: Username::try_from("tbt").unwrap(),
            password: Password::try_from("quest1!").unwrap(),
        };

        let json = serde_json::to_string(&data).unwrap();
        println!("{}", json);

        let raw = "{\"invitationToken\":[1,2,3,4],\"username\":\"tbt\",\"password\":\"quest1!\"}";
        let data = serde_json::from_str::<SignUpRequest>(raw).unwrap();
        println!("{:?}", data);
    }
}
