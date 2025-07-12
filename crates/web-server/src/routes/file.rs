use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Header, Status};
use rocket::{get, post, Response, State};
use uuid::Uuid;
use web_api_types::Result;

use crate::routes::UserContext;
use crate::services::FileSaver;
use crate::WebServerState;

#[post("/<id>", data = "<data>")]
pub async fn upload(_u: UserContext, server: &State<WebServerState>, id: Uuid, data: Data<'_>) -> Result<()> {
    server.file_saver.save_file(id, data.open(2.gigabytes())).await?;

    Ok(())
}

pub struct Resp<'r>(Response<'r>);

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Resp<'o> {
    fn respond_to(self, _req: &'r rocket::request::Request<'_>) -> rocket::response::Result<'o> {
        Ok(self.0)
    }
}

#[get("/<id>")]
pub async fn download(_u: UserContext, server: &State<WebServerState>, id: Uuid) -> Result<Resp<'_>> {
    let (size, data) = server.file_saver.open_file(id).await?;

    let mut response_builder = Response::build();
    response_builder.status(Status::Ok).header(ContentType::Binary);

    if let Some(size) = size {
        response_builder.header(Header::new("Content-Length", size.to_string()));
    }

    Ok(Resp(response_builder.streamed_body(data).finalize()))
}
