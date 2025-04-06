use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use web_api_types::{BlockId, Note, NoteId, NoteIds, Result, Space, SpaceId};

use crate::routes::UserContext;
use crate::WebServerState;

#[post("/space", data = "<data>")]
pub async fn add_space(server: &State<WebServerState>, user_context: UserContext, data: Json<Space>) -> Result<()> {
    Ok(server
        .data_service
        .add_space(data.into_inner(), user_context.user_id)
        .await?)
}

#[put("/space", data = "<data>")]
pub async fn update_space(server: &State<WebServerState>, user_context: UserContext, data: Json<Space>) -> Result<()> {
    Ok(server
        .data_service
        .update_space(data.into_inner(), user_context.user_id)
        .await?)
}

#[delete("/space/<space_id>")]
pub async fn remove_space(server: &State<WebServerState>, user_context: UserContext, space_id: SpaceId) -> Result<()> {
    Ok(server.data_service.remove_space(space_id, user_context.user_id).await?)
}

#[get("/spaces")]
pub async fn spaces(server: &State<WebServerState>, user_context: UserContext) -> Result<Json<Vec<Space>>> {
    Ok(Json(server.data_service.spaces(user_context.user_id).await?))
}

#[post("/note", data = "<data>")]
pub async fn add_note(
    server: &State<WebServerState>,
    user_context: UserContext,
    data: Json<Note>,
) -> Result<Json<BlockId>> {
    Ok(Json(
        server
            .data_service
            .add_note(data.into_inner(), user_context.user_id)
            .await?,
    ))
}

#[put("/note", data = "<data>")]
pub async fn update_note(server: &State<WebServerState>, user_context: UserContext, data: Json<Note>) -> Result<()> {
    Ok(server
        .data_service
        .update_note(data.into_inner(), user_context.user_id)
        .await?)
}

#[delete("/note/<note_id>")]
pub async fn remove_note(server: &State<WebServerState>, user_context: UserContext, note_id: NoteId) -> Result<()> {
    Ok(server.data_service.remove_note(note_id, user_context.user_id).await?)
}

#[post("/notes", data = "<data>")]
pub async fn notes(
    server: &State<WebServerState>,
    user_context: UserContext,
    data: Json<NoteIds>,
) -> Result<Json<Vec<Note>>> {
    Ok(Json(server.data_service.notes(&data.ids, user_context.user_id).await?))
}
