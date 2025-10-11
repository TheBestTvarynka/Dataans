use rocket::serde::json::Json;
use rocket::{State, get, post};
use web_api_types::{Result, User};

use crate::WebServerState;
use crate::routes::UserContext;

#[get("/")]
pub async fn get_user(_u: UserContext, server: &State<WebServerState>) -> Result<Option<Json<User>>> {
    Ok(server.user_service.user().await?.map(Json))
}

#[post("/", data = "<data>")]
pub async fn init_user(_u: UserContext, server: &State<WebServerState>, data: Json<User>) -> Result<()> {
    Ok(server.user_service.init(data.into_inner()).await?)
}
