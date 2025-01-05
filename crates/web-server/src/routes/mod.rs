mod auth;

pub use auth::*;
use rocket::get;

#[get("/health")]
pub fn health() -> &'static str {
    "ok"
}
