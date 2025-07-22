mod data;
mod file;

use std::str::FromStr;

pub use data::*;
pub use file::*;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rocket::get;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Responder, Response};
use serde::Deserialize;
use uuid::Uuid;

use crate::{Error, WebServerState};

pub const AUTH_HEADER_NAME: &str = "cf-access-jwt-assertion";

#[get("/")]
pub fn health() -> &'static str {
    "ok"
}

#[get("/auth")]
pub fn health_auth(_u: UserContext) -> &'static str {
    "auth_ok"
}

pub struct AuthorizationPage(&'static str);

impl<'r, 'o: 'r> Responder<'r, 'o> for AuthorizationPage {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .status(Status::Ok)
            .header(ContentType::HTML)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

#[get("/authorize.html")]
pub async fn cf_token(_u: UserContext) -> AuthorizationPage {
    AuthorizationPage(include_str!("../../authorize.html"))
}

#[derive(Debug)]
pub struct UserContext;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserContext {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        #![cfg_attr(feature = "dev", allow(unreachable_code))]
        #![cfg_attr(feature = "dev", allow(unused_variables))]

        // Only for local development!
        #[cfg(feature = "dev")]
        {
            return Outcome::Success(UserContext);
        }

        let Some(token) = req.headers().get_one(AUTH_HEADER_NAME) else {
            return Outcome::Error((
                Status::Unauthorized,
                Error::Unauthorized("missing authentication token"),
            ));
        };

        let state = match req
            .rocket()
            .state::<WebServerState>()
            .ok_or_else(|| Error::Internal("missing Rocket state"))
        {
            Ok(state) => state,
            Err(err) => return Outcome::Error((Status::InternalServerError, err)),
        };
        if let Err(err) = verify_auth_token(&state.cf_team_name, &state.cf_aud, token).await {
            return Outcome::Error((Status::Unauthorized, err));
        }

        Outcome::Success(UserContext)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CloudflareKey {
    kid: String,
    kty: String,
    alg: String,
    #[serde(rename = "use")]
    key_use: String,
    e: String,
    n: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CloudflarePublicCert {
    kid: String,
    cert: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CloudflareCerts {
    keys: Vec<CloudflareKey>,
    public_cert: CloudflarePublicCert,
    public_certs: Vec<CloudflarePublicCert>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Claims {
    aud: Vec<String>,
    country: String,
    email: String,
    exp: i64,
    iat: i64,
    identity_nonce: String,
    iss: String,
    nbf: i64,
    sub: Uuid,
    #[serde(rename = "type")]
    claim_type: String,
}

async fn verify_auth_token(team_name: &str, aud: &str, token: &str) -> Result<(), Error> {
    let header = decode_header(token).map_err(|_| Error::Unauthorized("invalid authentication token header"))?;
    let k_id = header
        .kid
        .ok_or_else(|| Error::Unauthorized("authentication token header does not contain 'kid' field"))?;

    let certs = reqwest::get(format!("https://{team_name}.cloudflareaccess.com/cdn-cgi/access/certs"))
        .await?
        .error_for_status()?
        .json::<CloudflareCerts>()
        .await?;

    let key = certs
        .keys
        .iter()
        .find(|k| k.kid == k_id)
        .ok_or_else(|| Error::Unauthorized("authentication token key id does not match any Cloudflare key"))?;
    let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)?;
    let mut validation = Validation::new(
        Algorithm::from_str(&key.alg)
            .map_err(|_| Error::Unauthorized("invalid algorithm in authentication token header"))?,
    );
    validation.set_audience(&[aud]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    trace!(?token_data, "Decoded authentication token");

    Ok(())
}
