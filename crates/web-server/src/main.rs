mod routes;

use rocket::{get, launch, routes};

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/health", routes![health, routes::sign_up,])
}
