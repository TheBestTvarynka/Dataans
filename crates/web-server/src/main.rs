#[macro_use]
extern crate tracing;

pub mod db;
mod error;
mod routes;
pub mod services;

use std::io;

pub use error::{Error, Result};
use rocket::{get, launch, routes};

const LOGGING_ENV_VAR_NAME: &str = "DATAANS_WEB_SERVER_LOG";
const DEFAULT_LOG_LEVEL: &str = "trace";

fn init_tracing() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let logging_filter: EnvFilter = EnvFilter::builder()
        .with_default_directive(DEFAULT_LOG_LEVEL.parse().expect("Default log level constant is bad."))
        .with_env_var(LOGGING_ENV_VAR_NAME)
        .from_env_lossy();

    let stdout_layer = tracing_subscriber::fmt::layer().pretty().with_writer(io::stdout);

    // TODO: add log file layer.
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(logging_filter)
        .init();
}

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[launch]
fn rocket() -> _ {
    init_tracing();

    rocket::build().mount("/health", routes![health, routes::sign_up,])
}
