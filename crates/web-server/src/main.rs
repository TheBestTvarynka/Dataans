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
use rocket::routes;
use sqlx::postgres::PgPoolOptions;

use crate::db::PostgresDb;
use crate::services::{Auth as AuthService, Data as DataService};

const DATABASE_URL: &str = "DATAANS_WEB_SERVER_DATABASE_URL";
const DATAANS_SERVER_ENCRYPTION_KEY: &str = "DATAANS_SERVER_ENCRYPTION_KEY";

pub struct State<D> {
    pub auth_service: AuthService<D>,
    pub data_service: DataService<D>,
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

        let server_encryption_key = hex::decode(
            std::env::var(DATAANS_SERVER_ENCRYPTION_KEY).expect("server encryption key env var should be set"),
        )
        .expect("server encryption key should be a valid hex string")
        .try_into()
        .expect("invalid server encryption key length");

        Self {
            auth_service: AuthService::new(Arc::clone(&db), server_encryption_key),
            data_service: DataService::new(Arc::clone(&db)),
        }
    }
}

impl Default for WebServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[rocket::main]
async fn main() -> std::result::Result<(), Box<rocket::Error>> {
    logging::init_tracing();

    let _rocket = rocket::build()
        .manage(WebServerState::new())
        .mount("/auth", routes![routes::sign_up, routes::sign_in])
        .mount(
            "/data",
            routes![routes::blocks, routes::operations, routes::add_operations,],
        )
        .mount("/health", routes![routes::health])
        .launch()
        .await?;

    Ok(())
}
