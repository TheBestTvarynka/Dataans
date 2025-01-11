use rocket::serde::json::Json;
use rocket::{post, State};
use uuid::Uuid;
use web_api_types::{Result, SignInRequest, SignInResponse, SignUpRequest};

use crate::WebServerState;

#[post("/sign-up", data = "<data>")]
pub async fn sign_up(server: &State<WebServerState>, data: Json<SignUpRequest>) -> Result<Json<Uuid>> {
    let SignUpRequest {
        invitation_token,
        username,
        password,
    } = data.into_inner();

    Ok(Json(
        server
            .auth_service
            .sign_up(invitation_token, &username, &password)
            .await?,
    ))
}

#[post("/sign-in", data = "<data>")]
pub async fn sign_in(server: &State<WebServerState>, data: Json<SignInRequest>) -> Result<SignInResponse> {
    let SignInRequest { username, password } = data.into_inner();

    let (user_id, token, expiration_date) = server.auth_service.sign_in(&username, &password).await?;

    Ok(SignInResponse {
        user_id,
        token,
        expiration_date,
    })
}
