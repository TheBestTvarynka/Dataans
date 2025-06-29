#![allow(async_fn_in_trait)]

#[macro_use]
extern crate tracing;

mod crypto;
pub mod db;
mod error;
mod logging;
mod routes;
pub mod services;

use std::env;
use std::sync::Arc;

pub use error::{Error, Result};
use rocket::routes;
use sqlx::postgres::PgPoolOptions;

use crate::db::PostgresDb;
use crate::services::{Auth as AuthService, Data as DataService};

const DATABASE_URL: &str = "DATAANS_WEB_SERVER_DATABASE_URL";
const SERVER_ENCRYPTION_KEY: &str = "DATAANS_SERVER_ENCRYPTION_KEY";

pub struct State<D, S> {
    pub auth_service: AuthService<D>,
    pub data_service: DataService<D>,
    pub file_saver: S,
}

#[cfg(feature = "fs")]
pub type WebServerState = State<PostgresDb, crate::services::Fs>;

#[cfg(feature = "fs")]
async fn prepare_file_loader() -> crate::services::Fs {
    use std::path::PathBuf;

    const FILES_DIR: &str = "DATAANS_WEB_SERVER_FILES_DIR";

    let files_dir = PathBuf::from(env::var(FILES_DIR).unwrap_or_else(|_| String::from("dist")));
    if !files_dir.exists() {
        std::fs::create_dir_all(&files_dir)
            .inspect_err(|err| {
                error!(?err, ?files_dir, "Failed to create files directory");
            })
            .expect("Failed to create files directory");
    }

    crate::services::Fs::new(files_dir)
}

#[cfg(feature = "tigris")]
async fn prepare_file_loader() -> crate::services::Tigris {
    use aws_config::{load_defaults, BehaviorVersion};

    const BUCKET_NAME: &str = "DATAANS_WEB_SERVER_S3_BUCKET";

    crate::services::Tigris::new(
        load_defaults(BehaviorVersion::latest()).await,
        env::var(BUCKET_NAME).expect("S3 bucket name env var should be set"),
    )
}

#[cfg(feature = "tigris")]
pub type WebServerState = State<PostgresDb, crate::services::Tigris>;

impl WebServerState {
    pub async fn new() -> WebServerState {
        let pool = PgPoolOptions::new()
            .max_connections(16)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect_lazy(&env::var(DATABASE_URL).expect("database url env var should be set"))
            .expect("can not connect to postgresql db");

        sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

        let db = Arc::new(PostgresDb::new(pool));

        let server_encryption_key =
            hex::decode(env::var(SERVER_ENCRYPTION_KEY).expect("server encryption key env var should be set"))
                .expect("server encryption key should be a valid hex string")
                .try_into()
                .expect("invalid server encryption key length");

        Self {
            auth_service: AuthService::new(Arc::clone(&db), server_encryption_key),
            data_service: DataService::new(Arc::clone(&db)),
            file_saver: prepare_file_loader().await,
        }
    }
}

#[rocket::main]
async fn main() -> std::result::Result<(), Box<rocket::Error>> {
    logging::init_tracing();

    let state = WebServerState::new().await;

    let _rocket = rocket::build()
        .manage(state)
        .mount("/auth", routes![routes::sign_up, routes::sign_in])
        .mount(
            "/data",
            routes![routes::blocks, routes::operations, routes::add_operations,],
        )
        .mount("/file", routes![routes::upload, routes::download])
        .mount("/health", routes![routes::health])
        .launch()
        .await?;

    Ok(())
}
