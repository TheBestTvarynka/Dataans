#[cfg(not(debug_assertions))]
pub fn init_tracing() {
    use std::io;

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    const LOGGING_ENV_VAR_NAME: &str = "DATAANS_WEB_SERVER_LOG";
    const DEFAULT_LOG_LEVEL: &str = "trace";

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

#[cfg(debug_assertions)]
pub fn init_tracing() {}