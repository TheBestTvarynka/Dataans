use rocket::serde::json::Json;
use rocket::{get, post, State};
use web_api_types::{Blocks, Operation, Result};

use crate::WebServerState;

#[get("/block?<items_per_block>")]
pub async fn blocks(server: &State<WebServerState>, items_per_block: usize) -> Result<Json<Blocks>> {
    Ok(Json(server.data_service.blocks(items_per_block).await?))
}

#[get("/operation?<operations_to_skip>")]
pub async fn operations(server: &State<WebServerState>, operations_to_skip: usize) -> Result<Json<Vec<Operation>>> {
    Ok(Json(server.data_service.operations(operations_to_skip).await?))
}

#[post("/operation", data = "<data>")]
pub async fn add_operations(server: &State<WebServerState>, data: Json<Vec<Operation>>) -> Result<()> {
    Ok(server.data_service.add_operations(data.into_inner()).await?)
}
