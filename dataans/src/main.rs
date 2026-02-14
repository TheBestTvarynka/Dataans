#![allow(clippy::empty_docs)]

#[macro_use]
extern crate tracing;

#[macro_use]
pub mod macros;

mod app;
mod app_info;
mod backend;
mod common;
mod dom;
mod notes;
mod spaces;
mod uuid;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    {
        use tracing_subscriber::EnvFilter;
        use tracing_subscriber::fmt::format::Pretty;
        use tracing_subscriber::prelude::*;
        use tracing_web::{MakeWebConsoleWriter, performance_layer};

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .without_time()
            .with_writer(MakeWebConsoleWriter::new());
        let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

        let logging_filter: EnvFilter = EnvFilter::builder()
            .with_default_directive("info".parse().expect("Default log level constant is bad."))
            .from_env_lossy();

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .with(logging_filter)
            .init();
    }

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
