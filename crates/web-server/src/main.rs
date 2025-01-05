#[macro_use]
extern crate tracing;

mod crypto;
pub mod db;
mod error;
mod logging;
mod routes;
pub mod services;

use std::sync::Arc;

pub use error::{Error, Result};
use rocket::{launch, routes};
use sqlx::postgres::PgPoolOptions;

use crate::db::PostgresDb;
use crate::services::Auth as AuthService;

const DATABASE_URL: &str = "DATAANS_WEB_SERVER_DATABASE_URL";

pub struct State<D> {
    pub auth_service: AuthService<D>,
}

pub type WebServerState = State<PostgresDb>;

impl WebServerState {
    pub fn new() -> WebServerState {
        let pool = PgPoolOptions::new()
            .max_connections(16)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect_lazy(&std::env::var(DATABASE_URL).expect("database url env var should be set"))
            .expect("can not connect to postgresql db");

        let db = Arc::new(PostgresDb::new(pool));

        Self {
            auth_service: AuthService::new(db),
        }
    }
}

#[launch]
fn rocket() -> _ {
    logging::init_tracing();

    rocket::build()
        .manage(WebServerState::new())
        .mount("/health", routes![routes::health, routes::sign_up,])
}
