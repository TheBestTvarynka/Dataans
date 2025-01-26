use rocket::serde::json::Json;
use rocket::{get, post, State};
use web_api_types::{BlockIds, BlockNotes, Result, SpaceId, SyncBlock};

use crate::routes::UserContext;
use crate::WebServerState;

#[get("/block/<space_id>")]
pub async fn blocks(
    server: &State<WebServerState>,
    user_context: UserContext,
    space_id: SpaceId,
) -> Result<Json<Vec<SyncBlock>>> {
    Ok(Json(server.sync_service.blocks(space_id, user_context.user_id).await?))
}

#[post("/block/notes", data = "<data>")]
pub async fn blocks_notes(
    server: &State<WebServerState>,
    user_context: UserContext,
    data: Json<BlockIds>,
) -> Result<Json<Vec<BlockNotes>>> {
    Ok(Json(
        server
            .sync_service
            .blocks_notes(&data.ids, user_context.user_id)
            .await?,
    ))
}
