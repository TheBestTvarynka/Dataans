use rocket::post;
use rocket::serde::json::Json;
use web_api_types::SignUpRequest;

#[post("/sign-up", data = "<data>")]
pub async fn sign_up(data: Json<SignUpRequest>) -> &'static str {
    "ok"
}
