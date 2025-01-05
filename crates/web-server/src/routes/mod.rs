mod auth;

pub use auth::*;
use rocket::get;

#[get("/")]
pub fn health() -> &'static str {
    "ok"
}
